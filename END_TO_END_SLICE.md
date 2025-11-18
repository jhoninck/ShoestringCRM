
# End-to-End Vertical Slice: `accounts` Query

This document shows a full vertical slice from:
GraphQL → Input Mapping → ORM → SQL → Postgres → GraphQL Response.

## 1. GraphQL Types

For the `accounts` vertical slice we use a Hygraph-style structure:

- `Node` interface for all top-level entities
- Relay-style `Connection` / `Edge` / `PageInfo`
- `Account` type implementing `Node`
- Root field: `accounts(where, orderBy, first, after)`

```graphql
scalar DateTime

"""
All top-level entities implement Node.
"""
interface Node {
  id: ID!
  createdAt: DateTime!
  updatedAt: DateTime!
}

"""
CRM Account entity.
"""
type Account implements Node {
  id: ID!
  createdAt: DateTime!
  updatedAt: DateTime!

  orgId: String!
  name: String!
  industry: String
  website: String
  phone: String
}

"""
Relay-style pagination metadata.
"""
type PageInfo {
  hasNextPage: Boolean!
  hasPreviousPage: Boolean!
  startCursor: String
  endCursor: String
}

"""
Edge wrapper for Account in a connection.
"""
type AccountEdge {
  cursor: String!
  node: Account!
}

"""
Connection wrapper for paginated Account lists.
"""
type AccountConnection {
  pageInfo: PageInfo!
  edges: [AccountEdge!]!
}


## 2. QueryRoot.accounts
(… same as provided in chat …)

## 3. Input Mapping (Where / OrderBy)
(… same as provided in chat …)

## 4. Execution Flow
(… same as provided in chat …)

Use this file as a developer reference for implementing vertical slices
for Contacts, Tickets, and SocialEvents.
