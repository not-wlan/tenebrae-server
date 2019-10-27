CREATE TYPE signature_state AS ENUM ('unverified', 'outdated', 'normal');

CREATE TABLE signatures (
    id SERIAL PRIMARY KEY,
    owner SERIAL REFERENCES signatures(id) NOT NULL,
    signature VARCHAR NOT NULL,
    file VARCHAR,
    state signature_state,
    name VARCHAR NOT NULL
)