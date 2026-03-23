
```mermaid
flowchart TB

%% =====================
%% BUSINESS (ROW)
%% =====================
subgraph BUSINESS["Business"]
  FUNNEL["Sales Funnel"]
end

%% =====================
%% BUSINESS SERVICES (ROW)
%% =====================
subgraph BIZ_SVC["Business Services"]
  SALES["Sales"] --- OPP["Opportunities"] --- CUST["Customers"]
end

%% =====================
%% APPLICATION SERVICES (ROW)
%% =====================
subgraph APP_SVC["Application Services"]
  FL["Flutter"] --- HF["Hasura Federation"] --- MAIL["Mail"] --- VOIP["VoIP"] --- SOCIAL["Social Media"]
end

%% =====================
%% APPLICATIONS (ROW)
%% =====================
subgraph APP["Applications"]
  Z["Zitadel (OIDC)"] --- G["Rust GraphQL API"] --- CMS["Hygraph / CMS"]
end

%% =====================
%% INFRA SERVICES (ROW)
%% =====================
subgraph INF_SVC["Infrastructure Services"]
  PG["PostgreSQL"] --- NATS["NATS Event Bus"] --- RFS["RustFS"] --- OBS["Observability"] --- GITOPS["GitOps"]
end

%% =====================
%% INFRA (ROW)
%% =====================
subgraph INF["Infrastructure"]
  K8S["Kubernetes / Multi-Cloud Platform"]
end

%% =====================
%% LAYER STACKING ONLY
%% =====================
BUSINESS --> BIZ_SVC
BIZ_SVC --> APP_SVC
APP_SVC --> APP
APP --> INF_SVC
INF_SVC --> INF
