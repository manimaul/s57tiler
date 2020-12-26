CREATE TABLE charts
(
    id        BIGSERIAL PRIMARY KEY,
    name      VARCHAR UNIQUE NOT NULL,
    scale     INTEGER        NOT NULL, -- DSPM_CSCL
    file_name VARCHAR        NOT NULL, -- DSID_DSNM
    updated   VARCHAR        NOT NULL, -- DSID_UADT
    issued    VARCHAR        NOT NULL  -- DSID_ISDT
);