-- Add migration script here

CREATE TABLE users (
	id Uuid NOT NULL PRIMARY KEY,
	first_name TEXT NOT NULL,
	last_name TEXT NOT NULL,
	password TEXT NOT NULL,
	email TEXT NOT NULL UNIQUE,
	phone TEXT NOT NULL,
	role TEXT NOT NULL,
	birth_date DATE NOT NULL,
	starting_date DATE NOT NULL DEFAULT CURRENT_DATE,
	created_at TIMESTAMP NOT NULL DEFAULT now() 
);

