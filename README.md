# CRM Platform (Rust + GraphQL + PostgreSQL + Flutter + ZITADEL)

A metadata-driven CRM platform integrating:

- **Rust** backend
- **GraphQL API** with async-graphql
- **Metadata-based ORM** (`ModelDef`, `FieldDef`)
- **PostgreSQL** via `sqlx`
- **ZITADEL** for identity, RBAC, and multi-tenant scoping
- **Flutter** for the frontend
- **Hygraph-style schema** for consistent CRUD
- **Social interaction ingestion** (posts, replies, DMs) feeding the CRM funnel

---

## 📊 Architecture Overview

![Architecture Diagram](docs/images/onboarding-diagram.png)

The diagram shows how identities, social interactions, CRM entities, and the API integrate into a single consistent system.

---

## ✨ Features

- Hygraph-like GraphQL CRUD (`Node`, `Connection`, `Where`, `OrderBy`)
- Metadata-driven ORM (add a new model by defining `ModelDef`)
- Generic SQL builder (SELECT + filtering + ordering + pagination)
- ZITADEL JWT → `Principal` → RBAC → scoped queries
- Vertical slices from GraphQL → ORM → SQL → Postgres
- Social → CRM funnel integration (Tickets, Leads, Activities)
- Flutter-friendly schema for reuse across views

---

## 📁 Repository Structure

```text
.
├── src/
│   ├── model/           # ModelDef + FieldDef + concrete models
│   ├── orm/             # Query language, SQL builder, RBAC
│   ├── graphql/         # Schema, types, inputs, resolvers
│   ├── security/        # ZITADEL JWT + Principal
│   ├── db/              # Connection & migrations
│   ├── main.rs          # Axum HTTP server
│   └── lib.rs
├── migrations/          # SQL migrations for Postgres
├── docs/
│   ├── ARCHITECTURE.md
│   ├── ONBOARDING.md
│   ├── WORKFLOW.md
│   ├── MODELDEF.md
│   ├── DIAGRAM.md
│   └── images/
│       ├── onboarding-diagram.png
│       ├── social-funnel-diagram.png
│       └── simple-social-crm-diagram.png
├── END_TO_END_SLICE.md  # Account vertical slice
├── CONTRIBUTING.md
├── LICENSE
└── README.md
```

---

## 🚀 Quickstart

### 1. Clone

```bash
git clone https://github.com/<YOUR_ORG>/crm-platform.git
cd crm-platform
```

### 2. Environment

Create a `.env` or export variables:

```bash
export DATABASE_URL=postgres://user:pass@localhost:5432/crm
export ZITADEL_ISSUER=https://<your-zitadel-domain>
export ZITADEL_AUDIENCE=crm-platform
```

### 3. Database

```bash
sqlx database create
sqlx migrate run
```

### 4. Run the API

```bash
cargo run
```

GraphQL endpoint: `http://localhost:8080/graphql`

---

## 🔐 Authentication & RBAC

- Flutter authenticates against **ZITADEL** using OIDC.
- Access tokens are sent as `Authorization: Bearer <token>` to this API.
- The server validates tokens, builds a `Principal` with:
  - `sub` (user id)
  - `org_id` (tenant id)
  - roles and permissions
- Every query and mutation is scoped via `with_scope(model, principal, filter)` so
  tenants see only their data.

See `src/orm/rbac.rs` and `docs/WORKFLOW.md` for details.

---

## 📚 Further Documentation

- [Architecture](docs/ARCHITECTURE.md)
- [Developer Onboarding](docs/ONBOARDING.md)
- [Workflow](docs/WORKFLOW.md)
- [ModelDef Specification](docs/MODELDEF.md)
- [Diagrams](docs/DIAGRAM.md)
- [Vertical Slice Example](END_TO_END_SLICE.md)

---

## 🧭 Roadmap

### Phase 1 — Core

- [x] ModelDef metadata system
- [x] SQL SELECT builder
- [x] Account vertical slice (`accounts` query)
- [x] Tenant scoping via ZITADEL Principal

### Phase 2 — CRM

- [ ] Contacts CRUD slice
- [ ] Tickets CRUD slice
- [ ] SocialEvent ingestion from connectors
- [ ] Sales funnel modelling (Leads / Opportunities)

### Phase 3 — Platform

- [ ] MinIO integration (files)
- [ ] NATS event bus
- [ ] Hygraph / content integration
- [ ] Advanced RBAC + audit logging

---

## 🧑‍💻 Contributing

Contributions welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for details.

---

## 📄 License

This project is licensed under the MIT License — see [LICENSE](LICENSE).
