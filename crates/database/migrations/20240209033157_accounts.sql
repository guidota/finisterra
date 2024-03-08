CREATE TABLE IF NOT EXISTS accounts 
(
    name        text primary key,
    email       text not null,
    password    text not null,
    pin         integer not null,
    created_at  timestamp not null default current_timestamp
);

CREATE TABLE IF NOT EXISTS characters
(
    name        text primary key,
    description text not null default '',
    level       integer not null default 1,
    exp         integer not null default 0,
    class_id    integer not null,
    race_id     integer not null,
    gender_id   integer not null,
    gold        integer not null default 0,
    map         integer not null default 1,
    x           integer not null default 50,
    y           integer not null default 50,
    created_at  timestamp not null default current_timestamp
);

CREATE TABLE IF NOT EXISTS character_attributes
(
    name            text primary key,
    strength        int not null,
    agility         int not null,
    intelligence    int not null,
    charisma        int not null,
    constitution    int not null,
    CONSTRAINT fk_character_name_1 FOREIGN KEY (name) REFERENCES characters(name)
);

CREATE TABLE IF NOT EXISTS character_statistics
(
    name        text primary key,
    health      int not null,
    mana        int not null,
    stamina     int not null,

    max_health  int not null,
    max_mana    int not null,
    max_stamina int not null,

    CONSTRAINT fk_character_name_2 FOREIGN KEY (name) REFERENCES characters(name)
);

CREATE TABLE IF NOT EXISTS character_equipment
(
    name        text primary key,
    weapon      int,
    shield      int,
    headgear    int,
    clothing    int,

    CONSTRAINT fk_character_name_3 FOREIGN KEY (name) REFERENCES characters(name)
);

CREATE TABLE IF NOT EXISTS character_look
(
    name        text primary key,
    body    int not null,
    face    int not null,
    skin    int not null,
    hair    int not null,

    CONSTRAINT fk_character_name_3 FOREIGN KEY (name) REFERENCES characters(name)
);

CREATE TABLE IF NOT EXISTS character_skills 
(
    name  text primary key,
    value bytea not null,

    CONSTRAINT fk_character_name_4 FOREIGN KEY (name) REFERENCES characters(name)
);

CREATE TABLE IF NOT EXISTS character_inventories
(
    name  text primary key,
    value bytea not null,

    CONSTRAINT fk_character_name_5 FOREIGN KEY (name) REFERENCES characters(name)
);

CREATE TABLE IF NOT EXISTS character_vaults (
    name  text primary key,
    value bytea not null,

    CONSTRAINT fk_character_name_6 FOREIGN KEY (name) REFERENCES characters(name)
);

CREATE TABLE IF NOT EXISTS character_spellbooks (
    name  text primary key,
    value bytea not null,

    CONSTRAINT fk_character_name_7 FOREIGN KEY (name) REFERENCES characters(name)
);

CREATE TABLE IF NOT EXISTS account_characters
(
    id              serial primary key,
    account_name    text not null,
    character_name  text not null,

    unique (character_name),
    CONSTRAINT fk_account_name FOREIGN KEY (account_name) REFERENCES accounts(name),
    CONSTRAINT fk_character_name_8 FOREIGN KEY (character_name) REFERENCES characters(name)
);
