# ModelDef & FieldDef

A `ModelDef` describes a database-backed model:

- `name`  — GraphQL model name (e.g. `Account`)
- `table` — database table name (e.g. `accounts`)
- `fields` — array of `FieldDef`

`FieldDef` includes:

- `name`          — GraphQL field name
- `column`        — database column name
- `scalar`        — scalar type (String, Int, DateTime, Json...)
- `is_primary_key`
- `is_filterable`
- `is_sortable`
- `is_writable`
- `has_default`
- `relation`      — relation metadata (optional)

Adding a new model:

1. Write SQL migration.
2. Define `FieldDef[]` and `ModelDef` in `src/model/<entity>.rs`.
3. Register it in `src/model/mod.rs`.
4. Implement GraphQL type + inputs and resolvers.
5. Add tests and docs.
