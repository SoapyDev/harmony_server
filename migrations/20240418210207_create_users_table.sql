-- Add migration script here

CREATE TABLE users (
	id SERIAL PRIMARY KEY,
	first_name TEXT NOT NULL,
	last_name TEXT NOT NULL,
	username TEXT NOT NULL UNIQUE,
	password TEXT NOT NULL,
	email TEXT NOT NULL UNIQUE,
	phone TEXT NOT NULL,
	role TEXT NOT NULL,
	birth_date DATE NOT NULL,
	starting_date DATE NOT NULL DEFAULT CURRENT_DATE,
	created_at TIMESTAMP NOT NULL DEFAULT now() 
);

