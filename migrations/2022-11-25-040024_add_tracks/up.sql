-- Your SQL goes here
create table tracks (
    id serial primary key,
    name text not null,
    audio_id serial references audios (id),
    time_length int,
    last_time int,
    last_playback timestamp,
    singer_id serial not null,
    album_id serial not null,
    foreign key (singer_id) references singers (id),
    foreign key (album_id) references albums (id)
);