CREATE TABLE contest_participates (
    id SERIAL PRIMARY KEY NOT NULL,
    user_id INT NOT NULL,
    FOREIGN KEY (user_id)
        REFERENCES users (id),
    contest_id INT NOT NULL,
    FOREIGN KEY (contest_id)
        REFERENCES contests (id),
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP
);
