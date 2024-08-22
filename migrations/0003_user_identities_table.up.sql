-- user_identities is used to track user sessions and identify unique users

-- this currently doesn't provide any additional feature, nor is required to
-- use the application, but it's useful for analytics

CREATE TABLE IF NOT EXISTS user_identities (
    id serial PRIMARY KEY,
    fingerprint varchar(128) NOT NULL UNIQUE,
    last_ip inet NOT NULL,
    last_version varchar(16) NOT NULL DEFAULT '1.0.0',
    session_count integer NOT NULL DEFAULT 1,
    platform varchar(64),
    -- Metadata
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_fingerprint ON user_identities(fingerprint);
CREATE INDEX IF NOT EXISTS idx_updated_at ON user_identities(updated_at);
CREATE INDEX IF NOT EXISTS idx_last_ip ON user_identities(last_ip);
