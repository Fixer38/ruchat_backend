-- Add migration script here
CREATE TABLE servers(
    id SERIAL,
    PRIMARY KEY (id),
    name varchar(255) NOT NULL UNIQUE,
    created_at timestamptz NOT NULL
);