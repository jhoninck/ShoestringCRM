```mermaid
flowchart TB

subgraph APP_SVC["Application Services"]
  FL["Flutter"]
  HF["Hasura Federation"]
  MAIL["Mail<br/>IMAP / SMTP"]
  VOIP["VoIP<br/>SIP / WebRTC"]
  SOCIAL["Social Media<br/>Posts / Comments / Messages"]
end

subgraph APP["Applications"]
  Z["Zitadel (OIDC)"]
  G["Rust GraphQL API"]
  CMS["Hygraph / CMS"]
end

subgraph INF_SVC["Infrastructure Services"]
  PG["PostgreSQL"]
  NATS["NATS Event Bus"]
  RFS["RustFS"]
  OBS["Observability<br/>Metrics • Logs • Tracing"]
  GITOPS["GitOps"]
end

subgraph INF["Infrastructure"]
  K8S["Kubernetes / Multi-Cloud Platform"]
end

APP_SVC --> APP
APP --> INF_SVC
INF_SVC --> INF
