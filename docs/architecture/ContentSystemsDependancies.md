## Content Systems

| Subsystem | PostgreSQL | RustFS | NATS | Redis | Purpose |
|---|---|---|---|---|---|
| Social Media Core | ✅ | ✅ | ✅ | ✅ | PostgreSQL for posts and profiles. RustFS for media. NATS for feed events. Redis for timeline cache. |
| Group / Community Platform | ✅ | ⚪ | ✅ | ✅ | PostgreSQL for membership and permissions. NATS for events. Redis for caching group views. |
| CMS / Content Management | ✅ | ✅ | ✅ | ✅ | PostgreSQL for pages and metadata. RustFS for assets. NATS for publish events. Redis for page cache. |
| Blog / Newsletter Platform | ✅ | ✅ | ✅ | ✅ | PostgreSQL for posts/subscribers. RustFS for assets. NATS for send workflows. Redis for caching. |
| File Storage / Uploads | ✅ | ✅ | ✅ | ✅ | PostgreSQL for ownership metadata. RustFS for objects. NATS for upload events. Redis for upload sessions. |
| Document Management | ✅ | ✅ | ✅ | ⚪ | PostgreSQL for document metadata. RustFS for binaries. NATS for indexing/OCR workflows. |
| Video Streaming / VOD | ✅ | ✅ | ✅ | ✅ | PostgreSQL for catalog metadata. RustFS for media segments. NATS for transcoding events. Redis for stream cache. |
| Live Streaming | ✅ | ✅ | ✅ | ✅ | PostgreSQL for stream metadata. RustFS for recordings. NATS for live events. Redis for realtime counters. |
| Audio / Podcast Platform | ✅ | ✅ | ✅ | ✅ | PostgreSQL for podcast metadata. RustFS for audio assets. NATS for publishing workflows. Redis for feed caching. |
| Ads / Campaign Platform | ✅ | ✅ | ✅ | ✅ | PostgreSQL for campaign metadata. RustFS for creatives. NATS for impression events. Redis for pacing counters. |
