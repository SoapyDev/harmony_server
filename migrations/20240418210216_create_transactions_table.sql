-- Add migration script here

CREATE TABLE transactions (
	id uuid NOT NULL,
	beneficiary_id INT NOT NULL,
	category_id SMALLINT DEFAULT 1,
	author TEXT NOT NULL,
	date TIMESTAMP NOT NULL,
	month_amount REAL NOT NULL,
	week_amount REAL NOT NULL,
	FOREIGN KEY (beneficiary_id) REFERENCES beneficiaries(id),
	PRIMARY KEY (id, beneficiary_id)
);
