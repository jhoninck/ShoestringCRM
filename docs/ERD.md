# Entity Relationship Diagram (Core CRM + Social)

The diagram below shows the **core ER model** for the CRM and social
modules, inspired by vtiger but adapted to the metadata-driven design.

```mermaid
erDiagram

    USERS ||--o{ ACCOUNTS : "owns / manages"
    USERS ||--o{ CONTACTS : "owns / manages"
    USERS ||--o{ LEADS : "owns / manages"
    USERS ||--o{ OPPORTUNITIES : "owns / manages"
    USERS ||--o{ TICKETS : "owns / manages"
    USERS ||--o{ ACTIVITIES : "assigned to"

    ACCOUNTS ||--o{ CONTACTS : "has"
    ACCOUNTS ||--o{ LEADS : "related to"
    ACCOUNTS ||--o{ OPPORTUNITIES : "related to"
    ACCOUNTS ||--o{ TICKETS : "related to"
    ACCOUNTS ||--o{ SOCIAL_EVENTS : "mentioned in"

    CONTACTS ||--o{ LEADS : "can be converted from"
    CONTACTS ||--o{ OPPORTUNITIES : "influences"
    CONTACTS ||--o{ TICKETS : "requests"
    CONTACTS ||--o{ SOCIAL_EVENTS : "engages via social"

    CAMPAIGNS ||--o{ LEADS : "generates"
    CAMPAIGNS ||--o{ OPPORTUNITIES : "influences"
    CAMPAIGNS ||--o{ ACTIVITIES : "contains"
    CAMPAIGNS ||--o{ SOCIAL_EVENTS : "drives"

    ACCOUNTS {
        uuid id
        text org_id
        text name
        text industry
        text website
        text phone
    }

    CONTACTS {
        uuid id
        text org_id
        uuid account_id
        text first_name
        text last_name
        text email
        text phone
    }

    LEADS {
        uuid id
        text org_id
        uuid account_id
        uuid owner_id
        text first_name
        text last_name
        text company
        text email
        text phone
        text status
        text source
    }

    OPPORTUNITIES {
        uuid id
        text org_id
        uuid account_id
        uuid contact_id
        uuid owner_id
        text name
        numeric amount
        text currency
        text stage
        int probability
    }

    TICKETS {
        uuid id
        text org_id
        uuid account_id
        uuid contact_id
        text subject
        text status
        text source
    }

    CAMPAIGNS {
        uuid id
        text org_id
        text name
        text type
        text status
    }

    ACTIVITIES {
        uuid id
        text org_id
        uuid owner_id
        uuid account_id
        uuid contact_id
        uuid lead_id
        uuid opportunity_id
        uuid ticket_id
        uuid campaign_id
        text subject
        text channel
        text activity_type
        text status
    }

    SOCIAL_EVENTS {
        uuid id
        text org_id
        text platform
        text channel
        text verb
        text direction
        text text
        float sentiment
        timestamp occurred_at
        uuid account_id
        uuid contact_id
        uuid ticket_id
    }
```

> Note: This ERD focuses on **core CRM entities** (Accounts, Contacts,
> Leads, Opportunities, Tickets, Campaigns, Activities) plus
> **SocialEvents**, not every vtiger table. It is meant as a clean,
> vtiger-inspired starting point for the new Rust + GraphQL platform.
