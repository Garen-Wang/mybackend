// @generated automatically by Diesel CLI.

diesel::table! {
    albums (id) {
        id -> Int4,
        name -> Text,
        singer_id -> Int4,
        last_playback -> Nullable<Timestamp>,
        agreed -> Bool,
    }
}

diesel::table! {
    audios (id) {
        id -> Int4,
        filepath -> Text,
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
    favorite_singers (id) {
        id -> Int4,
        user_id -> Int4,
        singer_id -> Int4,
    }
}

diesel::table! {
    favorite_tracks (id) {
        id -> Int4,
        user_id -> Int4,
        track_id -> Int4,
    }
}

diesel::table! {
    singers (id) {
        id -> Int4,
        name -> Text,
    }
}

diesel::table! {
    tracks (id) {
        id -> Int4,
        name -> Text,
        audio_id -> Int4,
        time_length -> Nullable<Int4>,
        last_time -> Nullable<Int4>,
        last_playback -> Nullable<Timestamp>,
        singer_id -> Int4,
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

diesel::joinable!(albums -> singers (singer_id));
diesel::joinable!(favorite_albums -> albums (album_id));
diesel::joinable!(favorite_albums -> users (user_id));
diesel::joinable!(favorite_singers -> singers (singer_id));
diesel::joinable!(favorite_singers -> users (user_id));
diesel::joinable!(favorite_tracks -> tracks (track_id));
diesel::joinable!(favorite_tracks -> users (user_id));
diesel::joinable!(tracks -> albums (album_id));
diesel::joinable!(tracks -> audios (audio_id));
diesel::joinable!(tracks -> singers (singer_id));

diesel::allow_tables_to_appear_in_same_query!(
    albums,
    audios,
    favorite_albums,
    favorite_singers,
    favorite_tracks,
    singers,
    tracks,
    users,
);
