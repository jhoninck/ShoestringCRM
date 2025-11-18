# Request → Response Workflow

1. **Client** (Flutter) sends GraphQL request with an access token.
2. **Security layer** validates the token using ZITADEL JWKS.
3. **Principal** is built (user id, org id, roles, permissions).
4. **Resolver** maps GraphQL inputs to ORM query types (`LogicalFilter`, `OrderBy`, etc.).
5. **RBAC**: `with_scope(model, principal, filter)` merges tenant and user filters.
6. **SQL builder** (`build_select`) constructs SQL with bindings.
7. **sqlx** executes the query against PostgreSQL.
8. **Rows** are mapped back to GraphQL objects and returned to the client.

This pattern applies to all vertical slices (Accounts, Contacts, Tickets, SocialEvents...).
