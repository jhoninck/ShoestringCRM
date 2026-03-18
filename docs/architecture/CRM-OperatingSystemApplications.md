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

%% Keep minimal relations
G --> Z
CMS --> Z

G --> PG
CMS --> PG
G --> NATS

PG --> K8S
NATS --> K8S
RFS --> K8S
OBS --> K8S
GITOPS --> K8S
