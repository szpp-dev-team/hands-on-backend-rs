CREATE TABLE contests (
    id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR NOT NULL UNIQUE,
    description VARCHAR NOT NULL,
    start_at TIMESTAMP NOT NULL,
    end_at TIMESTAMP NOT NULL,
    penalty INT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP
);