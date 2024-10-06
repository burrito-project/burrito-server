-- Add down migration script here

DROP TRIGGER IF EXISTS update_modified_time ON users;

DROP FUNCTION IF EXISTS internal.get_auth_user(p_username citext, p_password text);
DROP PROCEDURE IF EXISTS internal.change_user_password(p_username citext, p_password text);
DROP PROCEDURE IF EXISTS internal.create_user(p_username citext, p_password text, p_is_active boolean, p_is_staff boolean);

DROP INDEX IF EXISTS idx_users_is_staff;
DROP INDEX IF EXISTS idx_users_is_active;
DROP INDEX IF EXISTS idx_users_username;

DROP TABLE IF EXISTS users;

-- DROP EXTENSION IF EXISTS citext;

DROP EXTENSION IF EXISTS pgcrypto;
