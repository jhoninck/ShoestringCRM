# ORM Layer

This folder contains the **metadata-driven ORM**:

- `query.rs` – generic query language (filters, order, pagination, ScalarValue)
- `sql_builder.rs` – builds SELECT SQL from `ModelDef` + filters/order/page
- `rbac.rs` – Principal model and tenant scoping helpers
- `repo.rs` – thin wrapper around sqlx to execute BuiltQuery

Developers adding new models should NOT need to touch this layer
in most cases; they only define new `ModelDef`s in `src/model`.
