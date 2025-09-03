CREATE TABLE newsletter_subscribers (
    id UUID PRIMARY KEY,
    email VARCHAR NOT NULL UNIQUE,
    subscribed_at TIMESTAMPTZ NOT NULL,
    ip_address VARCHAR,
    location VARCHAR,
    user_agent TEXT
);

-- Add migration script here
