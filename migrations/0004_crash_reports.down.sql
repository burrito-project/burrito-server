
DROP TABLE IF EXISTS crash_reports;
DROP INDEX IF EXISTS idx_issuer;

DROP TRIGGER IF EXISTS update_modified_time ON crash_reports;
