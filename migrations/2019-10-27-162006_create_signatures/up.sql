CREATE TYPE api_key_state AS ENUM ('enabled', 'disabled', 'admin', 'moderator');

CREATE TABLE api_keys (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  key VARCHAR NOT NULL,
  state api_key_state NOT NULL,
  message VARCHAR
);

CREATE TABLE signatures (
    id SERIAL PRIMARY KEY,
    owner SERIAL REFERENCES api_keys(id) NOT NULL,
    signature VARCHAR NOT NULL,
    filename VARCHAR NOT NULL,
    filehash VARCHAR NOT NULL,
    name VARCHAR NOT NULL
);

CREATE TABLE signature_history (
    hid SERIAL PRIMARY KEY,
    id SERIAL,
    owner SERIAL REFERENCES api_keys(id) NOT NULL,
    signature VARCHAR NOT NULL,
    filename VARCHAR NOT NULL,
    filehash VARCHAR NOT NULL,
    name VARCHAR NOT NULL,
    created TIMESTAMP,
    created_by VARCHAR(32),
    deleted TIMESTAMP,
    deleted_by VARCHAR(32)
);

CREATE OR REPLACE FUNCTION signature_insert() RETURNS trigger AS
$$
  BEGIN
    INSERT INTO signature_history
      (id, owner, signature, filename, filehash, name, created, created_by)
    VALUES
      (NEW.id, NEW.owner, NEW.signature, NEW.filename, NEW.filehash, NEW.name,
       current_timestamp, current_user);
    RETURN NEW;
  END;
$$
LANGUAGE plpgsql;

CREATE TRIGGER signature_insert_trigger
AFTER INSERT ON signatures
  FOR EACH ROW EXECUTE PROCEDURE signature_insert();

CREATE OR REPLACE FUNCTION signature_delete() RETURNS trigger AS
$$
  BEGIN
    UPDATE signature_history
      SET deleted = current_timestamp, deleted_by = current_user
      WHERE deleted IS NULL and id = OLD.id;
    RETURN NULL;
  END;
$$
LANGUAGE plpgsql;

CREATE TRIGGER signature_delete_trigger
AFTER DELETE ON signatures
  FOR EACH ROW EXECUTE PROCEDURE signature_delete();

CREATE OR REPLACE FUNCTION signature_update() RETURNS trigger AS
$$
  BEGIN
    UPDATE signature_history
      SET deleted = current_timestamp, deleted_by = current_user
      WHERE deleted IS NULL and gid = OLD.gid;
    INSERT INTO signature_history
      (id, owner, signature, filename, filehash, name, created, created_by)
    VALUES
      (NEW.id, NEW.owner, NEW.signature, NEW.filename, NEW.filehash, NEW.name,
       current_timestamp, current_user);
    RETURN NEW;
  END;
$$
LANGUAGE plpgsql;

CREATE TRIGGER signature_update_trigger
AFTER UPDATE ON signatures
  FOR EACH ROW EXECUTE PROCEDURE signature_update();

CREATE UNIQUE INDEX unique_signature ON signatures (owner, signature, filehash);