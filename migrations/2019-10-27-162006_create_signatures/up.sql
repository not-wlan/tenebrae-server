CREATE TYPE signature_state AS ENUM ('unverified', 'outdated', 'normal');
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
    state signature_state NOT NULL,
    name VARCHAR NOT NULL,
    index INTEGER NOT NULL DEFAULT 0
);

CREATE UNIQUE INDEX unique_signature ON signatures (owner, signature, filehash, index);