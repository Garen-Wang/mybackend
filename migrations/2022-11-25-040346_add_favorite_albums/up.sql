-- Your SQL goes here
create table favorite_albums
(
    id   serial primary key,
    user_id serial not null,
    album_id serial not null,
    foreign key (user_id) references users (id),
    foreign key (album_id) references albums (id)
);