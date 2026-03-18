## AI / Data Systems

| Subsystem | PostgreSQL | RustFS | NATS | Redis | Purpose |
|---|---|---|---|---|---|
| AI Inference Gateway | ✅ | ✅ | ✅ | ✅ | PostgreSQL for model registry and usage logs. RustFS for models. NATS for async inference tasks. Redis for prompt caching. |
| AI Training Pipeline | ✅ | ✅ | ✅ | ✅ | PostgreSQL for experiment metadata. RustFS for datasets/models. NATS for training pipeline orchestration. Redis for job coordination. |
| Recommendation Engine | ✅ | ✅ | ✅ | ✅ | PostgreSQL for feature metadata. RustFS for datasets. NATS for event ingestion. Redis for feature cache. |
| Search / Indexing | ✅ | ✅ | ✅ | ✅ | PostgreSQL for index metadata. RustFS for corpus storage. NATS for indexing triggers. Redis for search cache. |
| Analytics / Event Processing | ✅ | ✅ | ✅ | ⚪ | PostgreSQL for aggregated metrics. RustFS for raw event archive. NATS for event ingestion pipelines. |
| Fraud / Abuse Detection | ✅ | ⚪ | ✅ | ✅ | PostgreSQL for cases/signals. NATS for detection events. Redis for realtime scoring cache. |
| Moderation / Trust & Safety | ✅ | ✅ | ✅ | ✅ | PostgreSQL for cases/policies. RustFS for evidence. NATS for moderation events. Redis for policy cache. |

Legend:

✅ = primary dependency  
⚪ = optional dependency  
❌ = generally not recommended
