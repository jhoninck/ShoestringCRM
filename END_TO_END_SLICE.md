
# End-to-End Vertical Slice: `accounts` Query

This document shows a full vertical slice from:
GraphQL → Input Mapping → ORM → SQL → Postgres → GraphQL Response.

## 1. GraphQL Types

For the `accounts` vertical slice we use a Hygraph-style structure:

- `Node` interface for all top-level entities
- Relay-style `Connection` / `Edge` / `PageInfo`
- `Account` type implementing `Node`
- Root field: `accounts(where, orderBy, first, after)`

```graphql
scalar DateTime
```

All top-level entities implement Node.

```
interface Node {
  id: ID!
  createdAt: DateTime!
  updatedAt: DateTime!
}
```


CRM Account entity.

```
type Account implements Node {
  id: ID!
  createdAt: DateTime!
  updatedAt: DateTime!

  orgId: String!
  name: String!
  industry: String
  website: String
  phone: String
}
```

Relay-style pagination metadata.

```
type PageInfo {
  hasNextPage: Boolean!
  hasPreviousPage: Boolean!
  startCursor: String
  endCursor: String
}
```

Edge wrapper for Account in a connection.
```
type AccountEdge {
  cursor: String!
  node: Account!
}

```
Connection wrapper for paginated Account lists.
```
type AccountConnection {
  pageInfo: PageInfo!
  edges: [AccountEdge!]!
}
```

## 2. QueryRoot.accounts

A typical root query field looks like this:

```
type Query {
  accounts(
    where: AccountWhereInput
    orderBy: [AccountOrderByInput!]
    first: Int
    after: String
  ): AccountConnection!
}
```

## 3. Input Mapping (Where / OrderBy)

ShoestringCRM uses a **metadata-driven filter DSL** to convert
GraphQL inputs into structured ORM filters.  
The process always follows the same pattern:

1. GraphQL input objects (`AccountWhereInput`, `AccountOrderByInput`)
2. Map them into `LogicalFilter` and `OrderBy` structs
3. The ORM converts these into SQL fragments based on the `ModelDef`

This makes all models behave uniformly without handwritten SQL.



### 3.1 GraphQL Input Objects

```graphql
```
String filter using GraphQL conventions similar to Hygraph.
```
input StringFilter {
  equals: String
  contains: String
  starts_with: String
  ends_with: String
}
```

Filter object for accounts.

```
input AccountWhereInput {
  name: StringFilter
  industry: StringFilter
}
```

Sorting for accounts.

```
enum AccountOrderField {
  name
  createdAt
  updatedAt
}

input AccountOrderByInput {
  field: AccountOrderField!
  direction: SortDirection = asc
}

enum SortDirection {
  asc
  desc
}
```

## 4. Execution Flow

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

