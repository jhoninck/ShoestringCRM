## CRM Systems

| Subsystem | PostgreSQL | RustFS | NATS | Redis | Purpose |
|---|---|---|---|---|---|
| CRM Contacts | ✅ | ⚪ | ✅ | ✅ | PostgreSQL for contact records and relationships. NATS for updates/events. Redis for lookup cache. |
| CRM Accounts / Organizations | ✅ | ⚪ | ✅ | ⚪ | PostgreSQL for company hierarchy and metadata. NATS for lifecycle events. |
| CRM Opportunities / Sales Pipeline | ✅ | ⚪ | ✅ | ⚪ | PostgreSQL for pipeline stages and deals. NATS for automation triggers. |
| Billing / Payments | ✅ | ⚪ | ✅ | ✅ | PostgreSQL for invoices and ledger metadata. NATS for payment events. Redis for idempotency/session caching. |
| Subscription Management | ✅ | ⚪ | ✅ | ✅ | PostgreSQL for plans and entitlements. NATS for lifecycle events. Redis for entitlement cache. |
| Customer Support / Ticketing | ✅ | ✅ | ✅ | ✅ | PostgreSQL for tickets and assignments. MinIO for attachments. NATS for escalation workflows. Redis for queue state. |
| Knowledge Base / Help Center | ✅ | ✅ | ✅ | ✅ | PostgreSQL for article metadata. MinIO for assets. NATS for indexing and publishing. Redis for page caching. |
| Contract / E-Signature Workflow | ✅ | ✅ | ✅ | ✅ | PostgreSQL for contract metadata. MinIO for signed documents. NATS for workflow orchestration. Redis for signing sessions. |
| Forms / Surveys | ✅ | ✅ | ✅ | ✅ | PostgreSQL for responses and forms. MinIO for attachments. NATS for submission events. Redis for draft/session caching. |
