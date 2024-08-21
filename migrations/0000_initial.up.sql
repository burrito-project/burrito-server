-- Pre defined
CREATE SCHEMA IF NOT EXISTS internal;

/*
 * The `public` *role* is automatically inherited by all other roles; we only want
 * specific roles to access our database so we revoke access to the public role.
*/
REVOKE ALL ON SCHEMA public FROM public;

ALTER DEFAULT PRIVILEGES IN SCHEMA public REVOKE ALL ON TABLES FROM public;
ALTER DEFAULT PRIVILEGES IN SCHEMA public REVOKE ALL ON SEQUENCES FROM public;
ALTER DEFAULT PRIVILEGES IN SCHEMA public REVOKE ALL ON FUNCTIONS FROM public;

ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT SELECT ON TABLES TO public;

-- Of course the owner should be able to do anything

GRANT ALL ON SCHEMA public TO admin;

CREATE SCHEMA IF NOT EXISTS public;

-- Tables that want to have updated_at column should setup a trigger like this:

-- CREATE OR REPLACE TRIGGER
-- update_modified_time BEFORE UPDATE ON <table>
-- FOR EACH ROW EXECUTE PROCEDURE trigger_set_timestamps();

CREATE OR REPLACE FUNCTION trigger_set_timestamps() RETURNS TRIGGER AS $$
BEGIN
    NEW.created_at = (CASE WHEN PG_OP = 'INSERT' THEN now() ELSE OLD.created_at END);
    NEW.updated_at = (CASE WHEN TG_OP = 'UPDATE' AND OLD.updated_at >= now() THEN OLD.updated_at + interval '1 millisecond' ELSE now() END);
    RETURN NEW;
END;
$$ LANGUAGE 'plpgsql' VOLATILE;
