CREATE TABLE IF NOT EXISTS accounts 
(
    name        TEXT NOT NULL PRIMARY KEY UNIQUE,
    mail        TEXT NOT NULL UNIQUE,
    password    TEXT NOT NULL,
    balance     INTEGER
);

CREATE TABLE IF NOT EXISTS characters 
(
    name TEXT NOT NULL PRIMARY KEY UNIQUE,
    account_name TEXT NOT NULL,
    race TEXT,
    price INTEGER,
    is_for_sale BOOLEAN,
    FOREIGN KEY (account_name) REFERENCES accounts (name)
);

INSERT OR IGNORE INTO accounts VALUES('guillotambo', 'mail', 'asd', 0);
INSERT OR IGNORE INTO characters VALUES('guillotambo', 'guillotambo', 'gnomo', null, false);
INSERT OR IGNORE INTO characters VALUES('guillotambo2', 'guillotambo', 'enano', null, false);

INSERT OR IGNORE INTO accounts VALUES('giyo', 'mail', 'asd', 0);
INSERT OR IGNORE INTO characters VALUES('giyo', 'giyo', 'gnomo', null, false);
INSERT OR IGNORE INTO characters VALUES('giyo2', 'giyo', 'enano', null, false);

INSERT OR IGNORE INTO accounts VALUES('guillo', 'mail', 'asd', 0);
INSERT OR IGNORE INTO characters VALUES('guillo', 'guillo', 'gnomo', null, false);
INSERT OR IGNORE INTO characters VALUES('guillo2', 'guillo', 'enano', null, false);
