-- Your SQL goes here
create table users
(
    id serial primary key,
    is_admin boolean not null default false,
    username text not null,
    email text not null unique,
    password text not null
);
