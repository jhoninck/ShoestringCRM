```mermaid
flowchart TB

%% =====================
%% BUSINESS (TOP)
%% =====================
subgraph BUSINESS["Business"]
    FUNNEL["Sales Funnel<br/>Sales → Opportunities → Customers"]
end

%% =====================
%% AI / DOMAIN
%% =====================
subgraph AI_DOMAIN["AI / Domain"]
    AI["AI Assistant<br/>Ollama + Guardrails"]
    RAG["RAG / Retrieval<br/>Policies / Redaction / Embeddings"]
end

%% =====================
%% APPLICATION / SERVICES
%% =====================
subgraph SERVICES["Application / Services"]
    Z["Zitadel (OIDC)"]
    F["Flutter"]
    G["Rust GraphQL API"]
    H["Hasura Federation"]
    CMS["Hygraph / CMS"]
    CC["Communication Channels<br/>Mail / VoIP / Social Media"]
end

%% =====================
%% INFRASTRUCTURE (LOWEST)
%% =====================
subgraph PLATFORM["Kubernetes / Multi-Cloud Platform"]
    P["PostgreSQL"]
    RFS["RustFS"]
    N["NATS Event Bus"]
    OBS["Observability"]
    GO["GitOps"]
end

%% =====================
%% FLOW
%% =====================

%% Business -> AI
FUNNEL --> AI
AI --> RAG

%% AI / services
AI --> G
RAG --> G

%% Clients / identity
Z --> G
F --> G

%% Service composition
G --> H
G --> CMS

%% Event / storage
CC --> N
H --> P
AI --> P
AI --> RFS
N --> AI

%% Ops / platform relations
GO -. deploys .-> G
GO -. deploys .-> H
GO -. deploys .-> AI

OBS -. monitors .-> G
OBS -. monitors .-> P
OBS -. monitors .-> N
OBS -. monitors .-> RFS
