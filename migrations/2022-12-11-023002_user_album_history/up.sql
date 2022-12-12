-- Your SQL goes here
create table user_album_history
(
    id serial primary key,
    user_id serial not null references users (id) on delete cascade,
    album_id serial not null references albums (id) on delete cascade,
    last_time int default 0,
    last_date timestamp default current_timestamp,
    unique(user_id, album_id)
);