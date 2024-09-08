-- crash_reports are a general purpose table to store crashes from all our applications

CREATE TABLE IF NOT EXISTS crash_reports (
    id serial PRIMARY KEY,
    issuer varchar(128) NOT NULL,
    error text NOT NULL,
    stacktrace text NOT NULL,
    -- Metadata
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_issuer ON crash_reports(issuer);

CREATE OR REPLACE TRIGGER update_modified_time BEFORE UPDATE ON crash_reports FOR EACH ROW EXECUTE PROCEDURE trigger_set_timestamps();
