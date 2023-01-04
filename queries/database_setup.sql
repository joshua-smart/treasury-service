CREATE TABLE IF NOT EXISTS transactions (
    id int NOT NULL UNIQUE,
    amount int NOT NULL,
    date DATETIME NOT NULL
);