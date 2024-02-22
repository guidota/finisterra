CREATE TABLE IF NOT EXISTS accounts 
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    mail        TEXT NOT NULL UNIQUE,
    password    TEXT NOT NULL
);
