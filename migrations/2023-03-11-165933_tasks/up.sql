CREATE TABLE tasks (
    id SERIAL PRIMARY KEY NOT NULL,
    author_id INT NOT NULL,
    FOREIGN KEY (author_id)
        REFERENCES users (id),
    contest_id INT NOT NULL,
    FOREIGN KEY (contest_id)
        REFERENCES contests (id),
    name VARCHAR NOT NULL,
    statement VARCHAR NOT NULL,
    constraints VARCHAR NOT NULL,
    input VARCHAR NOT NULL,
    output VARCHAR NOT NULL,
    score INT NOT NULL,
    time_limit INT NOT NULL,
    memory_limit INT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP
);