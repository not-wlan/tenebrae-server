CREATE TYPE api_key_state AS ENUM ('enabled', 'disabled');

CREATE TABLE api_keys (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  key VARCHAR NOT NULL,
  state api_key_state NOT NULL,
  message VARCHAR
)