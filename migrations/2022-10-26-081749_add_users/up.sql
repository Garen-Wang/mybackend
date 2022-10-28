-- Your SQL goes here
create extension if not exists "uuid-ossp";

create table users
(
    id uuid primary key default uuid_generate_v4(),
    email text not null unique,
    username text not null,
    password text not null,
    bio text,
    image text,
    created_at timestamp default current_timestamp not null,
    updated_at timestamp default current_timestamp not null
);