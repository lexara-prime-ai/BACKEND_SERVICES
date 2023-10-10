CREATE TABLE roles
(
    id         SERIAL PRIMARY KEY,
    code       VARCHAR(64)             NOT NULL UNIQUE,
    name       VARCHAR(128)            NOT NULL,
    created_at TIMESTAMP DEFAULT NOW() NOT NULL
)