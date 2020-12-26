-- Your SQL goes here
CREATE TABLE styles
(
    id BIGSERIAL PRIMARY KEY,
    name  VARCHAR UNIQUE NOT NULL,
    style JSONB          NOT NULL
);