CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    body TEXT NOT NULL,
    is_published BOOLEAN NOT NULL DEFAULT false,
    create_time TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    update_time TIMESTAMPTZ NOT NULL DEFAULT current_timestamp
);
