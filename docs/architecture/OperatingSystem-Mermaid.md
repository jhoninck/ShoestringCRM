```markdown
```mermaid
flowchart TB
    Z[Zitadel<br/>OIDC]
    F[Flutter]

    G[Rust GraphQL API]
    H[Hasura Federation]
    P[PostgreSQL]
    RFS[RustFS]
    N[NATS Event Bus]

    CMS[Hygraph / CMS]

    CC["Communication Channels<br/>Mail / VoIP / Social Media"]

    AI[AI Assistant<br/>Ollama + Guardrails]
    RAG["RAG / Retrieval<br/>Policies / Redaction / Embeddings"]

    SF[Sales Funnel]
    FUNNEL["Sales<br/>Opportunities<br/>Customers"]

    subgraph PLATFORM["Kubernetes / Multi-Cloud Platform"]
        K1[NATS]
        K2[PostgreSQL]
        K3[RustFS]
        K4[Observability]
        K5[GitOps]
    end

    Z --> G
    F --> H
    G --> H
    H --> P
    G --> CMS

    P --> AI
    P --> RFS
    CC --> N
    RFS -.-> N
    N --> AI
    AI --> RAG

    N --> SF
    CC --> SF
    RAG --> SF
    SF --> FUNNEL
