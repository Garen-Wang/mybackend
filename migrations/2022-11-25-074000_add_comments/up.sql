-- Your SQL goes here
create table comments (
    id serial primary key,
    album_id serial not null references albums (id) on delete cascade,
    author_id serial not null references users (id) on delete cascade,
    body text not null,
    created_at timestamp not null default current_timestamp
);

create index comments_album_id_idx on comments (album_id);

create index comments_author_id_idx on comments (author_id);