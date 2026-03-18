# Zero Trust Architecture

ShoestringCRM is designed around a **Zero Trust model**.

No component is implicitly trusted.  
All access between components is **explicit, authenticated, authorized, and audited**.

---

## Core Principle

Never trust, always verify.

Every request must:
- be authenticated
- be authorized
- be validated
- be auditable

This applies to:
- users
- services
- internal components
- infrastructure services
- observability

---

## Identity-Centric Architecture

All access is anchored in **Zitadel (OIDC)**.

- Users authenticate via Zitadel
- Services authenticate using service identities
- Tokens are required for all interactions
- No anonymous access is allowed

---

## Access Model

All interactions follow a strict flow:

Request  
→ Authentication (Zitadel)  
→ Authorization (Policy)  
→ Execution  
→ Audit  

No component can bypass this flow.

---

## Network Model

The system assumes:
- the network is untrusted
- internal traffic is not trusted by default
- every service call must be verified

There are no trusted internal zones.

---

## Service-to-Service Communication

Services do not communicate directly without:
- identity validation
- policy enforcement

Example:

Invalid:
GraphQL API → PostgreSQL

Valid:
GraphQL API → Authentication → Policy → PostgreSQL

---

## Data Access Principles

- no direct database access without authorization
- no direct object storage access without policy checks
- no implicit read permissions
- all access is scoped and validated

---

## Zero Trust Observability

Observability is implemented as a **policy-controlled telemetry system**.  
It is treated as untrusted and must follow the same Zero Trust rules.

### Principles

- Observability must authenticate for every request
- Observability has no direct access to storage or runtime environments
- All telemetry access is policy-controlled
- Every access is audited

---

### Data Flow

Direct access is not allowed:

Application → Disk → Observability

Instead:

Application → Structured Events → NATS → Observability Pipeline

---

### Access Model

All observability access follows:

Request  
→ Authentication (Zitadel)  
→ Policy Evaluation  
→ Redaction / Filtering  
→ Telemetry Storage / Response  

---

### Telemetry Types

- Metrics  
  Aggregated numerical data (latency, throughput, counts)

- Logs  
  Structured event records

- Traces  
  End-to-end request and event flows

---

### Data Classification

Allowed:
- system metrics
- event counts
- non-sensitive operational data

Restricted or Redacted:
- authentication tokens
- secrets and keys
- PII (unless explicitly allowed and masked)
- raw communication payloads
- AI prompts and responses (by default)

---

### Security Constraints

Observability must:
- use structured logging only
- apply redaction before storage
- avoid executing or evaluating input data
- use security-reviewed dependencies
- enforce retention limits
- restrict access via RBAC

---

### Rationale

Observability is a potential attack surface.

Past incidents have shown that:
- logging systems can execute code
- logs can expose sensitive data
- untrusted input can be abused

Observability must therefore be treated as:
A controlled data product, not a passive logging system.

---

## Event-Driven Enforcement

The platform uses an event-driven model:

Applications → NATS → Services

This ensures:
- controlled data flow
- visibility
- enforceable policies
- reduced direct coupling

---

## Security Properties

This architecture ensures:
- no hidden data access paths
- full traceability of actions
- strict least-privilege enforcement
- reduced attack surface
- containment of failures or exploits

---

## Summary

Zero Trust in ShoestringCRM means:
- identity is mandatory
- policies are enforced everywhere
- no direct access is allowed
- all actions are auditable

This applies to:
- applications
- services
- infrastructure
- observability

There are no exceptions.
