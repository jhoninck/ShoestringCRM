```mermaid
flowchart TB

%% =====================
%% BUSINESS (TOP)
%% =====================
    FUNNEL["Sales Funnel<br/>Sales → Opportunities → Customers"]

%% =====================
%% AI / DOMAIN
%% =====================
    AI["AI Assistant<br/>Ollama + Guardrails"]
    RAG["RAG / Retrieval<br/>Policies / Redaction / Embeddings"]

%% =====================
%% SERVICES (MIDDLE)
%% =====================
    G["Rust GraphQL API"]
    H["Hasura Federation"]
    CMS["Hygraph / CMS"]

%% =====================
%% DATA / EVENT LAYER
%% =====================
    P["PostgreSQL"]
    RFS["RustFS"]
    N["NATS Event Bus"]

%% =====================
%% CHANNELS / INPUT
%% =====================
    CC["Communication Channels<br/>Mail / VoIP / Social Media"]

%% =====================
%% CLIENT + IDENTITY
%% =====================
    Z["Zitadel (OIDC)"]
    F["Flutter"]

%% =====================
%% INFRASTRUCTURE (BOTTOM)
%% =====================
    subgraph PLATFORM["Kubernetes / Multi-Cloud Platform"]
        K1[NATS]
        K2[PostgreSQL]
        K3[RustFS]
        K4[Observability]
        K5[GitOps]
    end

%% =====================
%% FLOWS
%% =====================

    %% Client / Auth
    Z --> G
    F --> G

    %% Core services
    G --> H
    G --> CMS
    H --> P

    %% Data + events
    P --> AI
    P --> RFS
    RFS --> N
    CC --> N

    %% AI
    N --> AI
    AI --> RAG

    %% Business flow
    N --> FUNNEL
    CC --> FUNNEL
    RAG --> FUNNEL

    %% Platform
    PLATFORM --- P
    PLATFORM --- N
    PLATFORM --- RFS
