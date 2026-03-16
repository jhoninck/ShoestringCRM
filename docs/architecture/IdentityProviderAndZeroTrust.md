# Role of an OIDC Identity Provider (e.g., ZITADEL) in a Zero Trust Architecture

## Purpose

This document explains the role of an **OIDC Identity Provider (IdP)** such as **ZITADEL** in a **Zero Trust architecture**, and how it enables secure access to systems like:

- GraphQL APIs
- CRM systems
- CMS platforms
- PostgreSQL databases
- MinIO object storage
- NATS messaging
- Redis caching layers

In Zero Trust, the Identity Provider becomes the **root of trust** for both **human users** and **machine workloads**.

---

# Zero Trust Principle: Identity is the New Perimeter

Traditional security relied on network boundaries:

- internal network = trusted
- external network = untrusted

Zero Trust replaces this with **identity-based trust**.

Every access decision must answer:

- **Who is making the request?**
- **What are they allowed to do?**
- **Which resource are they trying to access?**
- **Under what conditions?**

OIDC providers like **ZITADEL** provide the **identity verification layer** required for this model.

---

# What an OIDC Provider Does

An **OIDC (OpenID Connect) Identity Provider** is responsible for:

1. Authenticating users or services
2. Issuing cryptographically signed identity tokens
3. Providing identity claims and permissions
4. Enabling services to verify identity without direct trust

Typical providers include:

- ZITADEL
- Keycloak
- Auth0
- Azure AD
- Okta

---

# High-Level Architecture
