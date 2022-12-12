-- Your SQL goes here
create table user_track_history
(
    id serial primary key,
    user_id serial not null references users (id) on delete cascade,
    track_id serial not null references tracks (id) on delete cascade,
    last_time int default 0,
    last_date timestamp default current_timestamp,
    unique(user_id, track_id)
);