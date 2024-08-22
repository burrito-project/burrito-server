-- This table keep tracks of client application releases. Not server releases.
-- It's mean to be used by the client application to check for updates.

-- The platform column aims to support platform specific updates.

CREATE TABLE IF NOT EXISTS app_versions (
    id serial PRIMARY KEY,
    semver varchar(16) NOT NULL,
    platform varchar(16) NOT NULL DEFAULT 'all',
    is_mandatory boolean NOT NULL,
    banner_url text,
    release_notes text, -- text in markdown format
    release_date timestamptz NOT NULL DEFAULT now(),
    -- Metadata
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now(),
    CHECK (semver ~ '^\d+\.\d+\.\d+$'),
    UNIQUE (semver),
    CONSTRAINT valid_platform CHECK (platform IN ('android', 'ios', 'web', 'all'))
);

CREATE OR REPLACE TRIGGER update_modified_time BEFORE UPDATE ON app_versions FOR EACH ROW EXECUTE PROCEDURE trigger_set_timestamps();
