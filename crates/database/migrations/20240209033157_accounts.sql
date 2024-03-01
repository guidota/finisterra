CREATE TABLE IF NOT EXISTS accounts 
(
    name        TEXT NOT NULL PRIMARY KEY UNIQUE,
    mail        TEXT NOT NULL UNIQUE,
    password    TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS characters 
(
    name TEXT NOT NULL PRIMARY KEY UNIQUE,
    account_name TEXT NOT NULL,
    race TEXT,
    FOREIGN KEY (account_name) REFERENCES accounts (name)
);

INSERT OR IGNORE INTO accounts VALUES('guillo', 'mail', 'asd');
INSERT OR IGNORE INTO characters VALUES('guillotambo', 'guillo', 'gnomo');
INSERT OR IGNORE INTO characters VALUES('guillotambo2', 'guillo', 'enano');
