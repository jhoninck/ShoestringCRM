CREATE TABLE users (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    org_id      TEXT NOT NULL,
    email       TEXT NOT NULL UNIQUE,
    display_name TEXT,
    role        TEXT,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE leads (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    org_id      TEXT NOT NULL,
    account_id  UUID REFERENCES accounts(id) ON DELETE SET NULL,
    owner_id    UUID REFERENCES users(id) ON DELETE SET NULL,
    first_name  TEXT,
    last_name   TEXT,
    company     TEXT,
    email       TEXT,
    phone       TEXT,
    status      TEXT,   -- e.g. New, Working, Qualified, Unqualified
    source      TEXT,   -- e.g. Web, Social, Referral
    description TEXT,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE opportunities (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    org_id          TEXT NOT NULL,
    account_id      UUID REFERENCES accounts(id) ON DELETE SET NULL,
    contact_id      UUID REFERENCES contacts(id) ON DELETE SET NULL,
    owner_id        UUID REFERENCES users(id) ON DELETE SET NULL,
    name            TEXT NOT NULL,
    amount          NUMERIC(18,2),
    currency        TEXT,
    stage           TEXT,  -- e.g. Qualification, Proposal, Closed Won, Closed Lost
    probability     INT,
    close_date      DATE,
    source          TEXT,
    description     TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE campaigns (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    org_id          TEXT NOT NULL,
    name            TEXT NOT NULL,
    type            TEXT,  -- e.g. Email, Social, Event
    status          TEXT,  -- e.g. Planned, Active, Completed
    start_date      DATE,
    end_date        DATE,
    budget          NUMERIC(18,2),
    expected_revenue NUMERIC(18,2),
    description     TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE activities (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    org_id          TEXT NOT NULL,
    owner_id        UUID REFERENCES users(id) ON DELETE SET NULL,
    account_id      UUID REFERENCES accounts(id) ON DELETE SET NULL,
    contact_id      UUID REFERENCES contacts(id) ON DELETE SET NULL,
    lead_id         UUID REFERENCES leads(id) ON DELETE SET NULL,
    opportunity_id  UUID REFERENCES opportunities(id) ON DELETE SET NULL,
    ticket_id       UUID REFERENCES tickets(id) ON DELETE SET NULL,
    campaign_id     UUID REFERENCES campaigns(id) ON DELETE SET NULL,
    subject         TEXT NOT NULL,
    channel         TEXT,  -- e.g. Email, Phone, Meeting, Social
    activity_type   TEXT,  -- e.g. Task, Event, Call
    status          TEXT,  -- e.g. Planned, Completed, Cancelled
    due_date        TIMESTAMPTZ,
    started_at      TIMESTAMPTZ,
    completed_at    TIMESTAMPTZ,
    notes           TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);
