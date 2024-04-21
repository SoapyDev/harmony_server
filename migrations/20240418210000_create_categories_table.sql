-- Add migration script here

CREATE TABLE categories (
	id SMALLSERIAL PRIMARY KEY,
	name VARCHAR(50) NOT NULL,
	description VARCHAR(255),
	weekly_limit REAL NOT NULL,
	monthly_limit REAL NOT NULL,
	count INT DEFAULT 0
);

INSERT INTO categories (name, description, weekly_limit, monthly_limit, count) VALUES ('Defaut', 'Category par default',25.00,50.00, 0);

COMMIT;
