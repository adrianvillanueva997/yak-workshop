-- Add migration script here
CREATE TABLE IF NOT EXISTS YAK (
    ID SERIAL PRIMARY KEY,
    NAME VARCHAR(255) NOT NULL,
    AGE real NOT NULL,
    AGE_LAST_SHAVED real
);
alter table YAK alter column AGE_LAST_SHAVED set default 0;
