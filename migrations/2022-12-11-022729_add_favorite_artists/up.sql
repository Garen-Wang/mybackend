-- Your SQL goes here

create table favorite_artists (
    id serial primary key,
    user_id serial not null references users (id) on delete cascade,
    artist_id serial not null references artists (id) on delete cascade,
    unique(user_id, artist_id)
);

create index favorite_artists_user_id_idx on favorite_artists (user_id);

create index favorite_artists_artist_id_idx on favorite_artists (artist_id);