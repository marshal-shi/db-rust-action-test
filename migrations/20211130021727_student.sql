-- Add migration script here
CREATE TABLE IF NOT EXISTS student (
       id serial primary key,
       name varchar(10)
);
