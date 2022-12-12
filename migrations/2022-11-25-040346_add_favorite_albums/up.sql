-- Your SQL goes here
create table favorite_albums (
    id serial primary key,
    user_id serial not null references users (id) on delete cascade,
    album_id serial not null references albums (id) on delete cascade,
    unique(user_id, album_id)
);

create index favorite_albums_user_id_idx on favorite_albums (user_id);

create index favorite_albums_album_id_idx on favorite_albums (album_id);