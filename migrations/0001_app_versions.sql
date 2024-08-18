CREATE TYPE platform_t AS ENUM ('android', 'ios', 'web');

CREATE TABLE IF NOT EXISTS app_versions (
    id serial PRIMARY KEY,
    semver varchar(16) NOT NULL,
    is_mandatory boolean NOT NULL,
    release_notes text,
    platform platform_t NOT NULL,
    release_date timestamp NOT NULL DEFAULT now(),
    updated_at timestamp NOT NULL DEFAULT now(),
    UNIQUE (platform, semver),
    CHECK (semver ~ '^\d+\.\d+\.\d+$'),
    CHECK (release_date > now())
);

CREATE TRIGGER update_modified_time BEFORE UPDATE ON app_versions FOR EACH ROW EXECUTE PROCEDURE trigger_set_timestamp();
