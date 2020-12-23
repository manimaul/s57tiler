-- Your SQL goes here
CREATE TABLE styles
(
    id     BIGSERIAL PRIMARY KEY,
    name   TEXT NOT NULL,
    style  JSONB NOT NULL
);