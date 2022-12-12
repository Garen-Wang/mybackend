-- Your SQL goes here
create table albums (
    id serial primary key,
    name text not null,
    artist_id serial references artists (id) not null,
    agreed boolean not null default false
);

create index albums_artist_id_idx on albums (artist_id);