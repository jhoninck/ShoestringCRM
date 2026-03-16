## Content Systems

| Subsystem | PostgreSQL | MinIO | NATS | Redis | Purpose |
|---|---|---|---|---|---|
| Social Media Core | ✅ | ✅ | ✅ | ✅ | PostgreSQL for posts and profiles. MinIO for media. NATS for feed events. Redis for timeline cache. |
| Group / Community Platform | ✅ | ⚪ | ✅ | ✅ | PostgreSQL for membership and permissions. NATS for events. Redis for caching group views. |
| CMS / Content Management | ✅ | ✅ | ✅ | ✅ | PostgreSQL for pages and metadata. MinIO for assets. NATS for publish events. Redis for page cache. |
| Blog / Newsletter Platform | ✅ | ✅ | ✅ | ✅ | PostgreSQL for posts/subscribers. MinIO for assets. NATS for send workflows. Redis for caching. |
| File Storage / Uploads | ✅ | ✅ | ✅ | ✅ | PostgreSQL for ownership metadata. MinIO for objects. NATS for upload events. Redis for upload sessions. |
| Document Management | ✅ | ✅ | ✅ | ⚪ | PostgreSQL for document metadata. MinIO for binaries. NATS for indexing/OCR workflows. |
| Video Streaming / VOD | ✅ | ✅ | ✅ | ✅ | PostgreSQL for catalog metadata. MinIO for media segments. NATS for transcoding events. Redis for stream cache. |
| Live Streaming | ✅ | ✅ | ✅ | ✅ | PostgreSQL for stream metadata. MinIO for recordings. NATS for live events. Redis for realtime counters. |
| Audio / Podcast Platform | ✅ | ✅ | ✅ | ✅ | PostgreSQL for podcast metadata. MinIO for audio assets. NATS for publishing workflows. Redis for feed caching. |
| Ads / Campaign Platform | ✅ | ✅ | ✅ | ✅ | PostgreSQL for campaign metadata. MinIO for creatives. NATS for impression events. Redis for pacing counters. |
