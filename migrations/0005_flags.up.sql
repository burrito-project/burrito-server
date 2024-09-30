-- Flags are tweakable application level booleans
-- * internal flags are only visible at a server level, never sent to users
-- * protected flags are meant to be only manipulated by superusers

-- Flags are CASE INSENSITIVE

CREATE EXTENSION IF NOT EXISTS citext;

CREATE TABLE IF NOT EXISTS flags (
    id serial PRIMARY KEY,
    name citext NOT NULL,
    display_name varchar(255) NOT NULL,
    value boolean NOT NULL,
    internal boolean NOT NULL DEFAULT false,
    protected boolean NOT NULL DEFAULT false,
    -- Metadata
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now(),

    CONSTRAINT unique_flag_name UNIQUE(name)
);

CREATE INDEX IF NOT EXISTS idx_flags_name ON flags(name);

CREATE OR REPLACE TRIGGER update_modified_time BEFORE UPDATE ON flags FOR EACH ROW EXECUTE PROCEDURE trigger_set_timestamps();
