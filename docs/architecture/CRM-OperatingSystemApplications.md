```mermaid
flowchart TB

subgraph APP["Applications"]
direction LR
  Z["Zitadel (OIDC)"] --> G["Rust GraphQL API"] --> CMS["Hygraph / CMS"]
end

subgraph INF_SVC["Infrastructure Services"]
direction LR
  PG["PostgreSQL"] --> NATS["NATS Event Bus"] --> RFS["RustFS"] --> OBS["Observability"] --> GITOPS["GitOps"]
end

subgraph INF["Infrastructure"]
direction LR
  K8S["Kubernetes / Multi-Cloud Platform"]
end

APP --> INF_SVC
INF_SVC --> INF

linkStyle 0 stroke-width:0px;
linkStyle 1 stroke-width:0px;
linkStyle 2 stroke-width:0px;
linkStyle 3 stroke-width:0px;
linkStyle 4 stroke-width:0px;
linkStyle 5 stroke-width:0px;
linkStyle 6 stroke-width:0px;
