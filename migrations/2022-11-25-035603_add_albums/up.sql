-- Your SQL goes here

create table albums
(
    id serial primary key,
    name text not null,
    singer_id serial references singers (id) not null,
    last_playback timestamp,
    agreed boolean not null default false
);
