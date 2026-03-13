# Architecture

ShoestringCRM is built as a **layered platform centered around a GraphQL API and an event-driven pipeline**.

Clients authenticate through **ZITADEL** and interact with the system via the **Rust GraphQL API**, which exposes the domain model and coordinates backend services.

## Core Stack

- **Flutter**  
  Multi-platform client interface.

- **ZITADEL**  
  Identity provider (OIDC) for authentication and RBAC.

- **Rust GraphQL API**  
  Central entry point for clients and integrations.

- **Hasura Federation**  
  GraphQL federation layer for schema composition and data access.

- **PostgreSQL**  
  Primary relational datastore for domain entities.

- **MinIO**  
  Object storage for documents, media, and AI-related artifacts.

- **NATS Event Bus**  
  Event backbone used to process and distribute CRM interactions.

## Communication Channels

External interactions enter the system through connectors:

- **Mail** (IMAP / SMTP)
- **VoIP** (SIP / WebRTC)
- **Social Media**
  - Social Post
  - Social Comment
  - Social Message

These connectors generate events that are published to the **NATS event bus**.

## AI Processing

An **AI assistant (Ollama)** processes CRM events and data using:

- **Guardrails** for policy enforcement
- **RAG / Retrieval** for contextual responses
  - policies
  - redaction
  - embeddings

AI insights are used to enrich CRM entities and interactions.

## CRM Funnel

All interactions ultimately feed the **CRM funnel**:

- **Sales**
- **Opportunities**
- **Customers**

Events from communication channels and AI processing help classify and promote leads through these stages.

## Infrastructure

The platform is designed to run on **Kubernetes in a multi-cloud environment**, providing:

- container orchestration
- observability
- GitOps-based deployment
- scalable event and storage infrastructure
