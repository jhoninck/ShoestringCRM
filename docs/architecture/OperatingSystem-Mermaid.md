```mermaid
flowchart BT

    %% =====================
    %% INFRASTRUCTURE BASE
    %% =====================
    subgraph PLATFORM["Kubernetes / Multi-Cloud Platform"]
        P["PostgreSQL"]
        N["NATS Event Bus"]
        RFS["RustFS"]
        OBS["Observability"]
        GO["GitOps"]
    end

    %% =====================
    %% SOFTWARE / SERVICES
    %% =====================
    Z["Zitadel (OIDC)"]
    F["Flutter"]
    G["Rust GraphQL API"]
    H["Hasura Federation"]
    CMS["Hygraph / CMS"]
    CC["Communication Channels<br/>Mail / VoIP / Social Media"]

    %% =====================
    %% AI / DOMAIN
    %% =====================
    AI["AI Assistant<br/>Ollama + Guardrails"]
    RAG["RAG / Retrieval<br/>Policies / Redaction / Embeddings"]

    %% =====================
    %% BUSINESS
    %% =====================
    FUNNEL["Sales Funnel<br/>Sales → Opportunities → Customers"]

    %% =====================
    %% FLOWS
    %% =====================

    PLATFORM --> Z
    PLATFORM --> F
    PLATFORM --> G
    PLATFORM --> H
    PLATFORM --> CMS
    PLATFORM --> CC
    PLATFORM --> AI
    PLATFORM --> RAG

    Z --> G
    F --> G
    G --> H
    G --> CMS

    CC --> N
    H --> P
    AI --> P
    AI --> RFS
    N --> AI
    AI --> RAG

    G --> FUNNEL
    AI --> FUNNEL
    RAG --> FUNNEL
    CC --> FUNNEL

    GO -. deploys .-> G
    GO -. deploys .-> H
    GO -. deploys .-> AI
    OBS -. monitors .-> G
    OBS -. monitors .-> P
    OBS -. monitors .-> N
    OBS -. monitors .-> RFS
