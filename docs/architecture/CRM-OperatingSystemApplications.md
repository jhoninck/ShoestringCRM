```mermaid
flowchart TB

%% Infrastructure (bottom)
subgraph INF["Infrastructure"]
  K8S["Kubernetes / Multi-Cloud Platform"]
end

%% Infrastructure Services (above infra)
subgraph INF_SVC["Infrastructure Services"]
  PG["PostgreSQL"]
  NATS["NATS Event Bus"]
  RFS["RustFS"]
  OBS["Observability"]
  GITOPS["GitOps"]
end

%% Applications (top)
subgraph APP["Applications"]
  Z["Zitadel (OIDC)"]
  G["Rust GraphQL API"]
  CMS["Hygraph / CMS"]
end

%% Force vertical layering
INF_SVC --> INF
APP --> INF_SVC
