-- Create users table for the panel login

CREATE EXTENSION IF NOT EXISTS citext;
CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE IF NOT EXISTS users (
    id serial PRIMARY KEY,
    username citext UNIQUE NOT NULL,
    display_name varchar(512) UNIQUE NOT NULL,
    password_hash varchar(255) NOT NULL,
    is_active boolean NOT NULL DEFAULT TRUE,
    is_staff boolean NOT NULL DEFAULT FALSE,
    last_login timestamptz DEFAULT NULL,
    -- Metadata
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now(),
    CHECK (char_length(password_hash) >= 60),
    CHECK (username != '' AND char_length(username) >= 4)
);


CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);
CREATE INDEX IF NOT EXISTS idx_users_is_active ON users(is_active);
CREATE INDEX IF NOT EXISTS idx_users_is_staff ON users(is_staff);


CREATE OR REPLACE PROCEDURE internal.create_user(
    p_username citext,
    p_display_name varchar(255),
    p_password text,
    p_is_active boolean,
    p_is_staff boolean
) AS $$
DECLARE
    v_password_hash varchar(255);
BEGIN
    v_password_hash := crypt(p_password, gen_salt('bf'));
    INSERT INTO users (username, display_name, password_hash, is_active, is_staff)
    VALUES (p_username, p_display_name, v_password_hash, p_is_active, p_is_staff);
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE PROCEDURE internal.change_user_password(
    p_username citext,
    p_password text
) AS $$
DECLARE
    v_password_hash varchar(255);
BEGIN
    v_password_hash := crypt(p_password, gen_salt('bf'));
    UPDATE users SET password_hash = v_password_hash WHERE username = p_username;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION internal.get_auth_user(
    p_username citext,
    p_password text
) RETURNS users AS $$
DECLARE
    v_user users%ROWTYPE;
BEGIN
    SELECT * INTO v_user FROM users WHERE username = p_username;

    IF v_user.password_hash = crypt(p_password, v_user.password_hash) THEN
        RETURN v_user;
    ELSE
        RETURN NULL;
    END IF;
EXCEPTION
    WHEN NO_DATA_FOUND THEN
        RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Apply trigger on users table for updating the modified time
CREATE OR REPLACE TRIGGER update_modified_time BEFORE UPDATE ON users FOR EACH ROW EXECUTE PROCEDURE trigger_set_timestamps();
