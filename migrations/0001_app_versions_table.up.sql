-- This table keep tracks of client application releases. Not server releases.
-- It's mean to be used by the client application to check for updates.

-- The platform_t ENUM aims to support platform specific updates.
DO $$
BEGIN
    CREATE TYPE platform_t AS ENUM ('android', 'ios', 'web', 'all');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

CREATE TABLE IF NOT EXISTS app_versions (
    id serial PRIMARY KEY,
    semver varchar(16) NOT NULL,
    platform platform_t NOT NULL DEFAULT 'all',
    is_mandatory boolean NOT NULL,
    banner_url text,
    release_notes text, -- text in markdown format
    release_date timestamp NOT NULL DEFAULT now(),
    -- Metadata
    created_at timestamp NOT NULL DEFAULT now(),
    updated_at timestamp NOT NULL DEFAULT now(),
    CHECK (semver ~ '^\d+\.\d+\.\d+$'),
    UNIQUE (semver)
);

CREATE OR REPLACE TRIGGER update_modified_time BEFORE UPDATE ON app_versions FOR EACH ROW EXECUTE PROCEDURE trigger_set_timestamps();
