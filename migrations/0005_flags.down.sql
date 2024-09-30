
DROP TABLE IF EXISTS flags;
DROP INDEX IF EXISTS idx_flags_name;

DROP TRIGGER IF EXISTS update_modified_time ON flags;

DROP EXTENSION IF EXISTS citext;
