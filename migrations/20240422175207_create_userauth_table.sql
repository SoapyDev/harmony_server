-- Add migration script here

CREATE TABLE authentications(
    user_id uuid NOT NULL,
    token uuid NOT NULL,
    expires_at timestamptz DEFAULT NOW() + INTERVAL '2 DAY',
    FOREIGN KEY (user_id) REFERENCES users(id),
    PRIMARY KEY (user_id,token)
);