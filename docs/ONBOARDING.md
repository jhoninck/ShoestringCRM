# Developer Onboarding

Welcome to the CRM Platform project.

## Prerequisites

- Rust (stable) and Cargo
- PostgreSQL running locally
- Basic understanding of GraphQL, async Rust, and Docker/Postgres
- A ZITADEL instance (or local dev configuration)

## Steps

1. Clone the repository and create a local branch.
2. Set `DATABASE_URL`, `ZITADEL_ISSUER`, and `ZITADEL_AUDIENCE`.
3. Run migrations: `sqlx database create && sqlx migrate run`.
4. Start the dev server: `cargo run`.
5. Open `http://localhost:8080/graphql` and run the `apiVersion` query.

Next, read:

- `END_TO_END_SLICE.md` — shows a full `accounts` vertical slice.
- `docs/MODELDEF.md` — explains how to define new models.
