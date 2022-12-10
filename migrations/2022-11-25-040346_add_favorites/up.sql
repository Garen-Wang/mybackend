-- Your SQL goes here
create table favorites (
    id serial primary key,
    user_id serial not null references users (id) on delete cascade,
    album_id serial not null references albums (id) on delete cascade,
    unique(user_id, album_id)
);

create index favorites_user_id_idx on favorites (user_id);

create index favorites_album_id_idx on favorites (album_id);