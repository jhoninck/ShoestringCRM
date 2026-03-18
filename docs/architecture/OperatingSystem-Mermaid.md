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
%% SERVICES
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
%% CHANNELS
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
%% FLOWS (TOP → DOWN)
%% =====================

    %% Business → AI
    FUNNEL --> AI
    AI --> RAG

    %% AI → Services
    AI --> G
    RAG --> G

    %% Services
    G --> H
    G --> CMS

    %% Services → Data
    H --> P

    %% Data / Events
    P --> RFS
    RFS --> N

    %% Channels → Events
    CC --> N

    %% Clients
    Z --> G
    F --> G

    %% Platform foundation
    PLATFORM --- P
    PLATFORM --- N
    PLATFORM --- RFS
