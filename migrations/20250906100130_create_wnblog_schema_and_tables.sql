-- Create wnblog schema for blog-related tables
CREATE SCHEMA IF NOT EXISTS wnblog;

-- Create subscribers table in wnblog schema
CREATE TABLE IF NOT EXISTS wnblog.subscribers (
    id UUID PRIMARY KEY,
    email VARCHAR NOT NULL UNIQUE,
    subscribed_at TIMESTAMPTZ NOT NULL,
    ip_address VARCHAR,
    location VARCHAR,
    user_agent TEXT
);

-- Add migration script here
