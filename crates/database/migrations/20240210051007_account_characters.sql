CREATE TABLE IF NOT EXISTS account_characters 
(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    account_id INTEGER,
    name TEXT NOT NULL UNIQUE,
    FOREIGN KEY (account_id) REFERENCES accounts (id) 
);

CREATE UNIQUE INDEX name_unique ON account_characters (lower(name))
