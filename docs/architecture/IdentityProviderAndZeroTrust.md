# OIDC Provider (ZITADEL) in a Zero Trust Architecture

## Purpose
An OIDC identity provider (e.g., ZITADEL) is the **root identity authority** in a Zero Trust system.  
It authenticates users and services, issues **signed identity tokens**, and allows every component to verify identity before granting access.

---

## Core Zero Trust Role

| Responsibility | Description |
|---|---|
| Authentication | Verifies users or services (SSO, MFA, passkeys, service credentials). |
| Token Issuance | Issues signed JWT access tokens containing identity and permissions. |
| Identity Claims | Provides roles, scopes, tenant IDs, and other authorization context. |
| Trust Anchor | All services trust tokens issued by the IdP rather than network location. |

Example token:

```json
{
  "sub": "user-123",
  "tenant": "tenant-a",
  "roles": ["sales"],
  "scope": ["crm.read"],
  "aud": "graphql-api"
}
```

---

## Position in the Architecture

```
Client
   │
   ▼
OIDC Provider (ZITADEL)
   │
   ▼
GraphQL API / Gateway
   │
   ├── CRM
   ├── CMS
   ├── PostgreSQL
   ├── MinIO
   ├── NATS
   └── Redis
```

Flow:

1. Client authenticates with the OIDC provider.
2. The provider issues a **signed access token**.
3. The client calls the API with the token.
4. The API verifies the token before accessing backend services.

---

## Integration With GraphQL

GraphQL acts as the **application access boundary**.

Responsibilities:

- validate the JWT issued by the OIDC provider
- enforce query / mutation authorization
- extract identity claims
- forward identity context to backend services

Example request header:

```
Authorization: Bearer <JWT>
```

Identity context forwarded internally:

```
x-user-id
x-tenant-id
x-roles
```

---

## Service-to-Service Identity

OIDC providers also issue tokens for **machine workloads**.

Example flow:

```
Service A → request token from IdP
Service A → call Service B with token
Service B → verify token
```

This removes trust based on **network location**.

---

## Authorization Model

Applications use token claims to enforce policy.

Typical claims:

- roles
- scopes
- tenant ID
- group membership

Example rule:

```
role: sales  → read CRM contacts
role: finance → read invoices
role: editor → modify CMS content
```

---

## Why This Enables Zero Trust

| Problem | Solution |
|---|---|
| Network trust | Identity-based verification |
| Shared credentials | Signed tokens per user/service |
| Long-lived sessions | Short-lived access tokens |
| Lateral movement | Independent token validation by every service |

---

## Key Takeaway

In a Zero Trust system:

- **ZITADEL (OIDC)** = identity authority  
- **GraphQL/API** = policy enforcement boundary  
- **Services & databases** = independently verify identity and enforce permissions  

Security decisions are based on **verified identity and explicit policy**, not on network trust.
