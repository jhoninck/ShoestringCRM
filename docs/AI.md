# AI Module

## Purpose

The AI module adds a controlled LLM integration to the CRM platform.  
Its purpose is to support CRM workflows such as summarization, drafting, classification, and guided assistance without bypassing the platform’s existing security and tenant isolation model.

The AI layer is designed to be:

- tenant-aware
- policy-controlled
- auditable
- compatible with local/self-hosted inference

## Initial Technology Choice

The first implementation targets **Ollama** as the inference runtime.

Reasons:

- local/self-hosted deployment
- simple HTTP API
- support for multiple open models
- reduced external dependency footprint
- suitable for privacy-sensitive and tenant-aware CRM workloads

## Security Model

The platform already uses OIDC-based authentication.  
This means the AI module does not introduce a separate identity model.

All AI requests must inherit the authenticated principal from the backend security layer.

The principal is expected to provide at least:

- user identity
- tenant / organization identity
- role or permission claims

This allows the AI module to enforce:

- tenant isolation
- role-based access control
- tool/function restrictions
- auditability of requests and responses

## Guardrail Architecture

The AI pipeline is designed as a guarded sequence:

1. request enters through authenticated backend layer
2. policy checks determine whether AI usage is allowed
3. relevant CRM data is fetched in tenant scope
4. sensitive data is minimized or redacted
5. sanitized prompt is sent to Ollama
6. output is validated before being returned
7. audit event is recorded

### Guardrail Layers

#### 1. Access Guardrails
Authorization is checked before any model invocation.

Examples:

- whether the authenticated user may use AI
- whether the user may access specific CRM entities
- whether a requested AI capability is allowed for the user role

#### 2. Data Guardrails
Only minimum necessary context should be sent to the model.

Examples:

- no unrestricted record dumps
- field-level filtering
- masking or redaction of sensitive fields
- strict tenant scoping

#### 3. Prompt Guardrails
System instructions are defined server-side and not by the client.

Examples:

- do not leak data across tenants
- do not fabricate actions
- do not claim database writes occurred unless confirmed
- stay within approved CRM use cases

#### 4. Tool Guardrails
If tool/function calling is later enabled, all tool calls must be re-authorized server-side.

The model must never directly mutate data without backend validation.

#### 5. Output Guardrails
Model output should be reviewed for unsafe or non-compliant content before returning it.

Examples:

- hidden fields leakage
- cross-tenant references
- unsupported claims
- disallowed actions

## Initial Module Layout

```text
src/ai/
  mod.rs
  service.rs
  ollama_client.rs
  policy.rs
  redaction.rs
  audit.rs
