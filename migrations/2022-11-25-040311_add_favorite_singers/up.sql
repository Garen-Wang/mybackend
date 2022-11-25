-- Your SQL goes here
create table favorite_singers
(
    id   serial primary key,
    user_id serial not null,
    singer_id serial not null,
    foreign key (user_id) references users (id),
    foreign key (singer_id) references singers (id)
);