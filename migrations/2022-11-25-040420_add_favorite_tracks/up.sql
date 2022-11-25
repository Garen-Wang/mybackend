-- Your SQL goes here
create table favorite_tracks
(
    id   serial primary key,
    user_id serial not null,
    track_id serial not null,
    foreign key (user_id) references users (id),
    foreign key (track_id) references tracks (id)
);