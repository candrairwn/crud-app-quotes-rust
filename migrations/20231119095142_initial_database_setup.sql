-- Add migration script here
CREATE TABLE IF NOT EXISTS quotes (
    id UUID PRIMARY KEY,
    book varchar NOT NULL,
    quote TEXT NOT NULL,
    inserted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL,
    UNIQUE (book, quote)
);