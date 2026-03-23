```mermaid
flowchart TB

%% =====================
%% BUSINESS SERVICES (TOP)
%% =====================
subgraph BIZ_SVC["Business Services"]
  LEADS["Lead Management"] --- PIPE["Pipeline Management"] --- REL["Customer Relationship Management"]
end

%% =====================
%% BUSINESS
%% =====================
subgraph BUSINESS["Business"]
  SALES["Sales"] --- OPP["Opportunities"] --- CUST["Customers"]
end

%% =====================
%% APPLICATION SERVICES
%% =====================
subgraph APP_SVC["Application Services"]
  FL["Flutter"] --- HF["Hasura Federation"] --- MAIL["Mail"] --- VOIP["VoIP"] --- SOCIAL["Social Media"] --- GR["GuardRails"] --- LLM["LLM / AI"]
end

%% =====================
%% APPLICATIONS
%% =====================
subgraph APP["Applications"]
  Z["Zitadel (OIDC)"] --- G["Rust GraphQL API"] --- CMS["Hygraph / CMS"]
end

%% =====================
%% INFRA SERVICES
%% =====================
subgraph INF_SVC["Infrastructure Services"]
  PG["PostgreSQL"] --- NATS["NATS Event Bus"] --- RFS["RustFS"] --- OBS["Observability"] --- GITOPS["GitOps"]
end

%% =====================
%% INFRA
%% =====================
subgraph INF["Infrastructure"]
  K8S["Kubernetes / Multi-Cloud Platform"]
end

%% =====================
%% LAYER STACKING ONLY
%% =====================
BIZ_SVC --> BUSINESS
BUSINESS --> APP_SVC
APP_SVC --> APP
APP --> INF_SVC
INF_SVC --> INF
