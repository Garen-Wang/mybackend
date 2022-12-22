-- Your SQL goes here
create table albums (
    id serial primary key,
    name text not null,
    last_playback timestamp not null default current_timestamp,
    artist_id serial references artists (id) not null,
    issued boolean not null default false
);

create index albums_artist_id_idx on albums (artist_id);