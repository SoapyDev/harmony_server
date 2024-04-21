-- Add migration script here

CREATE TABLE messages(
    id BIGSERIAL,
    beneficiary_id INT NOT NULL,
    message_type CHAR NOT NULL,
    content TEXT NOT NULL,
    date DATE NOT NULL,
    author TEXT NOT NULL,
    title TEXT DEFAULT '',
    FOREIGN KEY (beneficiary_id) REFERENCES beneficiaries(id),
    PRIMARY KEY(id, beneficiary_id)
);
