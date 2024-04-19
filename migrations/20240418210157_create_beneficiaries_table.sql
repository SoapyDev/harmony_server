-- Add migration script here


CREATE TABLE IF NOT EXISTS beneficiaries(
    id SERIAL PRIMARY KEY,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    email TEXT,
    phone TEXT,
    address TEXT,
    postal_code TEXT,
    kid INT NOT NULL DEFAULT 0,	
    adult INT NOT NULL DEFAULT 1,
    category_id SMALLINT NOT NULL DEFAULT 1,
    birth_date DATE,
    last_presence DATE,
    sexe TEXT,
    language TEXT,
    origin TEXT,
    city TEXT,
    study TEXT,
    income TEXT,
    family_situation TEXT,
    is_active BOOLEAN NOT NULL DEFAULT FALSE,
    is_sdf BOOLEAN NOT NULL DEFAULT FALSE,
    is_employed BOOLEAN NOT NULL DEFAULT FALSE,
    has_allergies BOOLEAN NOT NULL DEFAULT FALSE,
    has_general_note BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY(category_id) REFERENCES categories(id)
);
