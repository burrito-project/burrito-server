DROP INDEX IF EXISTS idx_fingerprint;
DROP INDEX IF EXISTS idx_last_seen;
DROP INDEX IF EXISTS idx_last_ip;

DROP TABLE IF EXISTS user_identities;

DROP TRIGGER IF EXISTS update_modified_time ON notification_ads;
