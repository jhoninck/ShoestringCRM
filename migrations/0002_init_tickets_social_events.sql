CREATE TYPE ticket_status AS ENUM ('OPEN', 'IN_PROGRESS', 'RESOLVED', 'CLOSED');
CREATE TYPE ticket_source AS ENUM ('EMAIL', 'PHONE', 'WEB', 'SOCIAL');

CREATE TABLE tickets (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    org_id      TEXT NOT NULL,
    account_id  UUID REFERENCES accounts(id) ON DELETE SET NULL,
    contact_id  UUID REFERENCES contacts(id) ON DELETE SET NULL,
    subject     TEXT NOT NULL,
    description TEXT,
    status      ticket_status NOT NULL DEFAULT 'OPEN',
    source      ticket_source NOT NULL DEFAULT 'WEB',
    solution_article_id TEXT,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TYPE social_platform AS ENUM ('TWITTER', 'LINKEDIN', 'MASTODON', 'INSTAGRAM', 'OTHER');
CREATE TYPE social_channel AS ENUM ('PUBLIC_POST', 'COMMENT', 'REPLY', 'DIRECT_MESSAGE');
CREATE TYPE social_verb AS ENUM ('POST', 'MENTION', 'REPLY', 'LIKE', 'SHARE', 'FOLLOW');
CREATE TYPE direction AS ENUM ('INBOUND', 'OUTBOUND');

CREATE TABLE social_events (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    org_id      TEXT NOT NULL,
    platform    social_platform NOT NULL,
    channel     social_channel NOT NULL,
    verb        social_verb NOT NULL,
    direction   direction NOT NULL,
    text        TEXT,
    sentiment   DOUBLE PRECISION,
    occurred_at TIMESTAMPTZ NOT NULL,
    account_id  UUID REFERENCES accounts(id) ON DELETE SET NULL,
    contact_id  UUID REFERENCES contacts(id) ON DELETE SET NULL,
    ticket_id   UUID REFERENCES tickets(id) ON DELETE SET NULL,
    raw_payload JSONB,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);
