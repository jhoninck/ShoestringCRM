# Contributing

Thank you for considering a contribution to this CRM platform.

## Development Workflow

1. **Fork** the repo and create a feature branch.
2. Run `cargo fmt` and `cargo clippy`.
3. Add or update tests where reasonable.
4. Update documentation (`docs/*.md`) if behavior or APIs change.
5. Open a Pull Request with a clear description and checklist.

### Commit Messages

Use short, descriptive messages:

- `feat: add ContactWhereInput`
- `fix: correct ticket status mapping`
- `docs: update onboarding`

### Code Style

- Rust 2021 edition
- `rustfmt` for formatting
- Avoid `unwrap()` / `expect()` in non-test code
- Errors should bubble up using `anyhow` or domain-specific errors

### Adding a New Model

1. Define the SQL in `migrations/`.
2. Create a `ModelDef` in `src/model/<name>.rs`.
3. Register it in `src/model/mod.rs`.
4. Add GraphQL object + inputs.
5. Implement the vertical slice (query and CRUD).
6. Add docs to `docs/MODELDEF.md` if it introduces a new pattern.
