# GraphQL in a Zero Trust Architecture

## Purpose

This document explains how **GraphQL fits into a Zero Trust architecture** and how it interacts securely with backend systems such as:

- CRM
- CMS
- PostgreSQL
- MinIO
- NATS
- Redis

GraphQL acts as the **application access boundary** that enforces authentication, authorization, and policy before any backend resource is accessed.

---

# Zero Trust Principles Applied to GraphQL

Zero Trust assumes:

- no implicit trust
- identity verification on every request
- strict authorization enforcement
- least privilege
- full observability

GraphQL becomes a **policy enforcement point** between clients and backend services.

---

# GraphQL Role in the Architecture

GraphQL sits between **clients** and **domain services / infrastructure**.

```
Client
   │
   ▼
Identity Provider (OIDC / ZITADEL / SSO)
   │
   ▼
GraphQL Gateway
   │
   ├── CRM service
   ├── CMS service
   ├── other domain services
   │
   ├── PostgreSQL
   ├── MinIO
   ├── NATS
   └── Redis
```

GraphQL is responsible for:

- verifying identity
- enforcing authorization
- validating queries
- forwarding identity context
- orchestrating backend calls

---

# Identity Enforcement

Every GraphQL request must include an **access token** issued by an identity provider.

Example request flow:

```
Client
  │
  ├── authenticate via OIDC
  │
  ▼
Access Token (JWT)
  │
  ▼
GraphQL API request
```

GraphQL verifies:

- token signature
- token expiration
- issuer
- audience
- scopes / roles

If verification fails → request rejected.

---

# Authorization in GraphQL

Authorization must occur at **three levels**.

## Query level

Control whether a user can run a query.

Example:

```graphql
query contacts {
  contacts { id name }
}
```

Only roles with `crm.read` permission may execute this query.

---

## Mutation level

Control who can perform actions.

Example:

```graphql
mutation deleteAccount {
  deleteAccount(id: "123")
}
```

Only users with an admin or privileged role should be able to execute destructive mutations.

---

## Field level

Control access to sensitive fields.

Example allowed:

```graphql
query {
  customer {
    name
    email
  }
}
```

Example blocked:

```graphql
query {
  customer {
    creditCardNumber
  }
}
```

GraphQL allows **fine-grained field-level authorization**, which aligns strongly with Zero Trust.

---

# Identity Propagation

GraphQL must pass verified identity to downstream services.

Example forwarded headers:

```
x-user-id
x-tenant-id
x-roles
x-scope
```

Services must not trust GraphQL blindly — they must also validate identity and enforce their own policies.

---

# Query Safety

GraphQL queries can be abused if unrestricted.

Required protections:

- query depth limits
- complexity limits
- rate limiting
- persisted queries
- request timeouts

Example attack:

```
deep nested queries
recursive fragments
expensive aggregations
```

Zero Trust requires **resource abuse protection** at the API boundary.

---

# Multi-Tenant Isolation

GraphQL must enforce tenant isolation.

Typical token claims:

```json
{
  "sub": "user123",
  "tenant": "tenantA",
  "roles": ["sales"]
}
```

GraphQL must ensure queries only return data belonging to the tenant.

Backend systems should enforce this as well.

---

# GraphQL Interaction With Backend Systems

## GraphQL + CRM

GraphQL exposes CRM data through controlled queries.

Controls include:

- role-based access
- ownership checks
- export restrictions
- audit logging

GraphQL prevents direct CRM database exposure.

---

## GraphQL + CMS

GraphQL manages content access for:

- editors
- reviewers
- publishers
- readers

Policies control:

- draft visibility
- workflow transitions
- publication permissions

---

## GraphQL + PostgreSQL

GraphQL does **not replace database security**.

PostgreSQL should still enforce:

- least privilege roles
- row-level security
- tenant isolation

GraphQL connects using restricted database roles.

---

## GraphQL + MinIO

GraphQL should **not proxy file uploads**.

Instead:

1. GraphQL generates a **signed upload URL**
2. client uploads directly to MinIO
3. GraphQL stores metadata in PostgreSQL

Example:

```
Client
   │
   ▼
GraphQL → signed URL
   │
   ▼
Upload to MinIO
   │
   ▼
Metadata stored in PostgreSQL
```

---

## GraphQL + NATS

GraphQL mutations often generate domain events.

Example mutation:

```graphql
mutation createPost {
  createPost(...)
}
```

GraphQL publishes event:

```
post.created
```

Services subscribe to events:

- notifications
- moderation
- analytics
- recommendation engines

---

## GraphQL + Redis

Redis supports GraphQL operations through:

- query caching
- session storage
- rate limiting
- subscription state

Example:

```
GraphQL Query
   │
   ▼
Redis Cache
   │
   ▼
PostgreSQL
```

Redis improves performance without weakening Zero Trust policies.

---

# Observability

GraphQL must log:

- query execution
- mutation execution
- authorization failures
- unusual query patterns
- performance anomalies

Logs should feed into security monitoring systems.

---

# Security Controls Checklist

A Zero Trust GraphQL deployment should include:

Authentication
- OIDC token validation
- signature verification
- expiration checks

Authorization
- query-level permissions
- mutation permissions
- field-level restrictions

API Protection
- query complexity limits
- rate limiting
- request validation

Identity Propagation
- verified identity passed to services

Backend Enforcement
- database security
- storage policies
- messaging permissions

Observability
- audit logs
- query monitoring
- anomaly detection

---

# Summary

GraphQL fits into Zero Trust as the **secure orchestration layer** between clients and backend systems.

Its responsibilities include:

| Responsibility | Description |
|---|---|
| Authentication | Verify identity on every request |
| Authorization | Enforce permissions on queries, mutations, and fields |
| Policy Enforcement | Apply access rules before backend access |
| Identity Propagation | Pass verified identity to downstream services |
| Query Safety | Prevent resource abuse and malicious queries |
| Observability | Log all operations for monitoring and auditing |

In a Zero Trust architecture, GraphQL becomes the **application security boundary** that ensures backend systems are accessed only through verified and authorized operations.
