-- Add migration script here

CREATE TABLE transactions (
	id uuid DEFAULT gen_randaom_uuid(),
	beneficiary_id INT NOT NULL,
	category_id SMALLINT NOT NULL,
	username TEXT NOT NULL,
	date TIMESTAMP NOT NULL,
	month_amount REAL NOT NULL,
	week_amount REAL NOT NULL,
	FOREIGN KEY (beneficiary_id) REFERENCES beneficiaries(id),
	FOREIGN KEY (category_id) REFERENCES categories(id),
	PRIMARY KEY (id, beneficiary_id)
);
