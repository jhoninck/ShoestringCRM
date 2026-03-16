## Communications Systems

| Subsystem | PostgreSQL | MinIO | NATS | Redis | Purpose |
|---|---|---|---|---|---|
| VoIP / Telephony | ✅ | ✅ | ✅ | ✅ | PostgreSQL for call metadata and routing. MinIO for recordings. NATS for call events. Redis for SIP session state. |
| Chat / Messaging | ✅ | ✅ | ✅ | ✅ | PostgreSQL for conversation metadata. MinIO for attachments. NATS for message events. Redis for presence/state. |
| Email Platform | ✅ | ✅ | ✅ | ⚪ | PostgreSQL for mailbox metadata. MinIO for attachments. NATS for mail processing pipelines. |
| Notifications Service | ✅ | ⚪ | ✅ | ✅ | PostgreSQL for notification history. NATS for fan-out events. Redis for rate limiting/deduplication. |
| Mobile Push | ✅ | ⚪ | ✅ | ✅ | PostgreSQL for device tokens. NATS for push events. Redis for rate limiting and queue cache. |
| Voice / Video Conferencing | ✅ | ✅ | ✅ | ✅ | PostgreSQL for rooms and participants. MinIO for recordings. NATS for call events. Redis for presence/state. |
| Presence / Realtime Status | ⚪ | ❌ | ✅ | ✅ | NATS for presence events. Redis for realtime presence state. |
