```mermaid
flowchart TB

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
APP_SVC --> APP
APP --> INF_SVC
INF_SVC --> INF
