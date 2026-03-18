```mermaid
flowchart TB

subgraph "Applications"
  Z["Zitadel (OIDC)"]
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

PG --> K8S
NATS --> K8S
RFS --> K8S
OBS --> K8S
GITOPS --> K8S
