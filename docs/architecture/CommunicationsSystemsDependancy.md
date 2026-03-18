## Communications Systems

| Subsystem | PostgreSQL | RustFS | NATS | Redis | Purpose |
|---|---|---|---|---|---|
| VoIP / Telephony | ✅ | ✅ | ✅ | ✅ | PostgreSQL for call metadata and routing. RustFS for recordings. NATS for call events. Redis for SIP session state. |
| Chat / Messaging | ✅ | ✅ | ✅ | ✅ | PostgreSQL for conversation metadata. RustFS for attachments. NATS for message events. Redis for presence/state. |
| Email Platform | ✅ | ✅ | ✅ | ⚪ | PostgreSQL for mailbox metadata. RustFS for attachments. NATS for mail processing pipelines. |
| Notifications Service | ✅ | ⚪ | ✅ | ✅ | PostgreSQL for notification history. NATS for fan-out events. Redis for rate limiting/deduplication. |
| Mobile Push | ✅ | ⚪ | ✅ | ✅ | PostgreSQL for device tokens. NATS for push events. Redis for rate limiting and queue cache. |
| Voice / Video Conferencing | ✅ | ✅ | ✅ | ✅ | PostgreSQL for rooms and participants. RustFS for recordings. NATS for call events. Redis for presence/state. |
| Presence / Realtime Status | ⚪ | ❌ | ✅ | ✅ | NATS for presence events. Redis for realtime presence state. |
