-- Add migration script here
CREATE TABLE IF NOT EXISTS connections (
	id uuid NOT NULL,
	username TEXT NOT NULL,
	created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
	expires_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP + interval '12 hours',
	FOREIGN KEY (username) REFERENCES users(username),
	PRIMARY KEY (id, username)
);
