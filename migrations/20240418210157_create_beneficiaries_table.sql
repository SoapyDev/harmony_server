-- Add migration script here


CREATE TABLE IF NOT EXISTS beneficiaries(
    id SERIAL PRIMARY KEY,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    email TEXT NOT NULL,
    phone TEXT NOT NULL,
    address TEXT NOT NULL,
    postal_code TEXT NOT NULL,
    kid INT DEFAULT 0,	
    adult INT DEFAULT 1,
    category_id SMALLINT DEFAULT 1,
    birth_date DATE DEFAULT CURRENT_DATE,
    last_presence DATE NOT NULL,
    sexe TEXT NOT NULL,
    language TEXT NOT NULL,
    origin TEXT NOT NULL,
    city TEXT NOT NULL,
    study TEXT NOT NULL,
    income TEXT NOT NULL,
    family_situation TEXT NOT NULL,
    is_active BOOLEAN DEFAULT FALSE,
    is_sdf BOOLEAN DEFAULT FALSE,
    is_employed BOOLEAN DEFAULT FALSE,
    has_allergies BOOLEAN DEFAULT FALSE,
    has_general_note BOOLEAN DEFAULT FALSE,
    FOREIGN KEY(category_id) REFERENCES categories(id)
);
