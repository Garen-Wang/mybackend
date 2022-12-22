// @generated automatically by Diesel CLI.

diesel::table! {
    albums (id) {
        id -> Int4,
        name -> Text,
        last_playback -> Timestamp,
        artist_id -> Int4,
        issued -> Bool,
    }
}

diesel::table! {
    artists (id) {
        id -> Int4,
        name -> Text,
    }
}

diesel::table! {
    comments (id) {
        id -> Int4,
        album_id -> Int4,
        author_id -> Int4,
        body -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    favorite_albums (id) {
        id -> Int4,
        user_id -> Int4,
        album_id -> Int4,
    }
}

diesel::table! {
    favorite_artists (id) {
        id -> Int4,
        user_id -> Int4,
        artist_id -> Int4,
    }
}

diesel::table! {
    tracks (id) {
        id -> Int4,
        name -> Text,
        last_playback -> Timestamp,
        url -> Text,
        artist_id -> Int4,
        album_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        is_admin -> Bool,
        username -> Text,
        email -> Text,
        password -> Text,
    }
}

diesel::joinable!(albums -> artists (artist_id));
diesel::joinable!(comments -> albums (album_id));
diesel::joinable!(comments -> users (author_id));
diesel::joinable!(favorite_albums -> albums (album_id));
diesel::joinable!(favorite_albums -> users (user_id));
diesel::joinable!(favorite_artists -> artists (artist_id));
diesel::joinable!(favorite_artists -> users (user_id));
diesel::joinable!(tracks -> albums (album_id));
diesel::joinable!(tracks -> artists (artist_id));

diesel::allow_tables_to_appear_in_same_query!(
    albums,
    artists,
    comments,
    favorite_albums,
    favorite_artists,
    tracks,
    users,
);
