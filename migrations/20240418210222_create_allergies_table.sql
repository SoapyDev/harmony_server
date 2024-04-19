CREATE TABLE allergies(
    beneficiary_id INT NOT NULL,
    name TEXT NOT NULL,
    danger_level TEXT NOT NULL,
    comment TEXT,
    FOREIGN KEY (beneficiary_id) REFERENCES beneficiaries(id),
    PRIMARY KEY (beneficiary_id, name)
);
