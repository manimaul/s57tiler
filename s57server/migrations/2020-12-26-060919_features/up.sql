CREATE TABLE features
(
    id       BIGSERIAL PRIMARY KEY,
    layer    VARCHAR                       NOT NULL,
    geom     GEOMETRY(GEOMETRY, 4326)      NOT NULL,
    props    JSONB                         NOT NULL,
    chart_id BIGINT REFERENCES charts (id) NOT NULL
);
CREATE INDEX features_gist ON features USING GIST (geom);
CREATE INDEX features_idx ON features (id);
