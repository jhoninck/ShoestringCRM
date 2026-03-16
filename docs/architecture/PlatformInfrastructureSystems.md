## Platform Systems

| Subsystem | PostgreSQL | MinIO | NATS | Redis | Purpose |
|---|---|---|---|---|---|
| Identity / Access Management | ✅ | ⚪ | ✅ | ✅ | PostgreSQL for users and roles. NATS for audit events. Redis for session/token caching. |
| SSO / OIDC Provider | ✅ | ⚪ | ✅ | ✅ | PostgreSQL for auth metadata. NATS for login events. Redis for session/token storage. |
| API Gateway / Edge API | ✅ | ⚪ | ✅ | ✅ | PostgreSQL for client metadata. NATS for async job delegation. Redis for rate limiting and caching. |
| GraphQL / Hypergraph API | ✅ | ⚪ | ✅ | ✅ | PostgreSQL for data access. NATS for subscriptions/events. Redis for query caching. |
| Workflow / Automation Engine | ✅ | ⚪ | ✅ | ⚪ | PostgreSQL for workflow definitions/state. NATS for triggers. |
| Scheduler / Job System | ✅ | ⚪ | ✅ | ✅ | PostgreSQL for job metadata. NATS for distributed execution. Redis for locks/ephemeral state. |
| Webhooks / Integration Layer | ✅ | ⚪ | ✅ | ✅ | PostgreSQL for webhook registry. NATS for event fan-out. Redis for retry scheduling. |
| Import / Export Service | ✅ | ✅ | ✅ | ✅ | PostgreSQL for job metadata. MinIO for source/export files. NATS for pipeline orchestration. Redis for progress state. |
| Reporting / BI Exports | ✅ | ✅ | ✅ | ⚪ | PostgreSQL for reports. MinIO for generated exports. NATS for generation triggers. |
| Backup / Archive Catalog | ✅ | ✅ | ✅ | ⚪ | PostgreSQL for backup metadata. MinIO for archives. NATS for backup events. |
| IoT / Device Registry | ✅ | ✅ | ✅ | ✅ | PostgreSQL for device registry. MinIO for firmware. NATS for telemetry. Redis for device state cache. |
| Geospatial / Location Services | ✅ | ✅ | ✅ | ✅ | PostgreSQL for geospatial queries. MinIO for geodata. NATS for location events. Redis for proximity cache. |

Legend:

✅ = primary dependency  
⚪ = optional dependency  
❌ = generally not recommended
