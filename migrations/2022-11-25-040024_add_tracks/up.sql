-- Your SQL goes here
create table tracks (
    id serial primary key,
    name text not null,
    time_length int not null,
    last_time int default 0,
    last_playback timestamp default current_timestamp,
    artist_id serial not null references artists (id) on delete cascade,
    album_id serial not null references albums (id) on delete cascade
);

create index tracks_artist_id_idx on tracks (artist_id);

create index tracks_album_id_idx on tracks (album_id);