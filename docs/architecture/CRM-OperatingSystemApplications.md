```mermaid
flowchart TB

subgraph "Applications"
  Z["Zitadel (OIDC)"]
  G["Rust GraphQL API"]
  CMS["Hygraph / CMS"]
end

subgraph "Infrastructure Services"
  PG["PostgreSQL"]
  NATS["NATS Event Bus"]
  RFS["RustFS"]
  OBS["Observability"]
  GITOPS["GitOps"]
end

subgraph "Infrastructure"
  K8S["Kubernetes / Multi-Cloud Platform"]
end

%% Zero trust: applications go via Zitadel
G --> Z
CMS --> Z

%% Applications use infrastructure services
G --> PG
G --> NATS
CMS --> PG

%% Infra services run on platform
PG --> K8S
NATS --> K8S
RFS --> K8S
OBS --> K8S
GITOPS --> K8S
