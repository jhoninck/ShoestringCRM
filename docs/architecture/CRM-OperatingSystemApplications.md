```mermaid
flowchart TB

subgraph "Applications"
  Z["Zitadel (OIDC)"]
  F["Flutter"]
  G["Rust GraphQL API"]
  H["Hasura Federation"]
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

Z --> PG
G --> PG
G --> NATS
H --> PG

PG --> K8S
NATS --> K8S
RFS --> K8S
OBS --> K8S
GITOPS --> K8S
