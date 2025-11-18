# Architecture

The platform is built as a set of layers:

1. **Flutter** — multi-platform client.
2. **ZITADEL** — identity & access management (OIDC).
3. **Rust GraphQL API** — single entrypoint for all clients.
4. **ORM / ModelDef** — metadata-driven model layer.
5. **PostgreSQL** — relational storage.
6. **(Future) Hygraph / CMS** — content and templates.
7. **Social connectors** — ingest social events into the CRM funnel.

The goal is to define domain entities once via `ModelDef` and make them
available across:

- GraphQL types & inputs
- SQL queries
- RBAC scoping
- Analytics and reporting
