-- Add migration script here

CREATE TABLE messages(
    id BIGSERIAL PRIMARY KEY,
    message_type CHAR NOT NULL,
    content TEXT NOT NULL,
    date DATE NOT NULL,
    author TEXT NOT NULL,
    title TEXT NOT NULL
);
