## Execution Flow

This section describes how a GraphQL query like:

```graphql
query {
  accounts(
    first: 20
    after: "0"
    where: { name: { contains: "Acme" } }
    orderBy: [{ field: name, direction: asc }]
  ) {
    edges { cursor node { id name } }
    pageInfo { hasNextPage endCursor }
  }
}

```

moves through the ShoestringCRM stack—from the resolver, through RBAC,
to the ORM, to SQL, and back into a GraphQL response.

### Overview Pipeline


```
GraphQL Request
     ↓
GraphQL Resolver (QueryRoot)
     ↓
Input Mapping (WhereInput → LogicalFilter)
     ↓
RBAC Injection (with_scope)
     ↓
ORM Query Builder (ModelDef-driven)
     ↓
SQLx Prepared Query
     ↓
PostgreSQL
     ↓
Rows → Model instances
     ↓
Connection / Edges → GraphQL Response
```

Every entity (Account, Lead, Opportunity, etc.) follows the same pattern.


### Step-by-step Breakdown

#### Step 1 — GraphQL Resolver

The resolver receives four parameters:

    where

    orderBy

    first

    after

Example (Accounts):

async fn accounts(
    &self,
    ctx: &Context<'_>,
    where_: Option<AccountWhereInput>,
    order_by: Option<Vec<AccountOrderByInput>>,
    first: Option<i32>,
    after: Option<String>,
) -> Result<AccountConnection> {
    resolve_accounts(ctx, where_, order_by, first, after).await
}

The resolver does not build SQL directly—this is handled generically.
Step 2 — Convert WhereInput → LogicalFilter

let user_filter = to_account_filter(where_)?;

Example input:

where: { name: { contains: "Acme" } }

Becomes internal DSL:

LogicalFilter::Simple(
  FieldFilter {
    field: "name",
    op: Contains,
    value: "Acme"
  }
)

This is still model-agnostic.
Step 3 — RBAC Injection

The RBAC layer inserts organization/document-level constraints based on the
ZITADEL-derived Principal:

let scoped_filter = with_scope(ACCOUNT_MODEL, principal, user_filter);

Example RBAC policy injected:

LogicalFilter::And([
  FieldFilter { field: "orgId", op: Equals, value: "acme-org-123" },
  FieldFilter { field: "name", op: Contains, value: "Acme" }
])

This ensures:

    User can only query accounts in their org_id

    No leaking across tenants

Step 4 — OrderBy Mapping

let order_by_vec = order_by
    .unwrap_or_default()
    .into_iter()
    .map(to_account_order_by)
    .collect::<Result<Vec<_>, _>>()?;

Example:

orderBy: [{ field: name, direction: asc }]

Becomes:

OrderBy { field: "name", direction: Asc }

Step 5 — Pagination Handling

GraphQL Relay-style cursor pagination:

    after = row offset (String)

    first = page size

let offset = after.map(|s| s.parse::<i64>().unwrap_or(0)).unwrap_or(0);
let limit = first.unwrap_or(20) as i64;
let page = Page { limit, offset };

Step 6 — ORM Executes Query

The ORM converts:

    ModelDef

    LogicalFilter

    OrderBy

    Page

into SQL:

let rows = fetch_many(
    pool,
    ACCOUNT_MODEL,
    scoped_filter.as_ref(),
    &order_by_vec,
    Some(page),
).await?;

Generated SQL (simplified):

SELECT id, org_id, name, industry, created_at, updated_at
FROM accounts
WHERE org_id = $1 
  AND name ILIKE '%' || $2 || '%'
ORDER BY name ASC
OFFSET $3
LIMIT $4

This works for every model—no handwritten SQL needed.
Step 7 — Map Rows → GraphQL Nodes

let node = Account {
    id: ID(row.get::<Uuid>("id").to_string()),
    org_id: row.get("org_id"),
    name: row.get("name"),
    industry: row.get("industry"),
    created_at: row.get::<DateTime<Utc>>("created_at").to_rfc3339(),
    updated_at: row.get::<DateTime<Utc>>("updated_at").to_rfc3339(),
};

Step 8 — Wrap in Connection/Edges

let cursor = (offset + idx + 1).to_string();

edges.push(AccountEdge {
    cursor,
    node,
});

Pagination metadata:

let page_info = PageInfo {
  has_next_page,
  has_previous_page,
  start_cursor,
  end_cursor,
};

4.3 Final GraphQL Response

{
  "accounts": {
    "pageInfo": {
      "hasNextPage": true,
      "endCursor": "20"
    },
    "edges": [
      {
        "cursor": "1",
        "node": {
          "id": "e7b0e7...",
          "name": "Acme Corporation",
          "industry": "Software"
        }
      }
    ]
  }
}

Every vertical slice (Accounts, Leads, Opportunities, Activities) follows this exact pipeline.
