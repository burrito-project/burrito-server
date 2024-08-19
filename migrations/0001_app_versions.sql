CREATE TABLE IF NOT EXISTS app_versions (
    id serial PRIMARY KEY,
    semver varchar(16) NOT NULL,
    is_mandatory boolean NOT NULL,
    banner_url text,
    release_notes text,
    release_date timestamp NOT NULL DEFAULT now(),
    updated_at timestamp NOT NULL DEFAULT now(),
    CHECK (semver ~ '^\d+\.\d+\.\d+$'),
    UNIQUE (semver)
);

CREATE OR REPLACE TRIGGER update_modified_time BEFORE UPDATE ON app_versions FOR EACH ROW EXECUTE PROCEDURE trigger_set_timestamp();
