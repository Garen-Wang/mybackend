use actix_web::web;

use crate::{
    album::handler::*, artist::handler::*, comment::handler::*, favorite::handler::*,
    healthcheck::handler::*, user::handler::*,
};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/healthcheck1").route("", web::get().to(healthcheck1)));
    cfg.service(web::scope("/healthcheck2").route("", web::get().to(healthcheck2)));

    cfg.service(
        web::scope("/auth")
            .route("login", web::post().to(login))
            .route("register", web::post().to(register)),
    );

    cfg.service(
        web::scope("/user")
            .route("/all", web::get().to(get_all_users)) // need admin token
            .route("/{user_id}", web::delete().to(remove_user)) // need admin token
            .route("", web::get().to(get_myself)) // need token
            .route("", web::put().to(update)) // need token
    );

    cfg.service(
        web::scope("/artist")
            .route("/all", web::get().to(get_all_artists))
            .route("/{artist_name}", web::get().to(search_artists_by_name))
            .route("/{artist_id}", web::delete().to(delete_artist))
            .route("", web::post().to(add_artist))
    );

    cfg.service(
        web::scope("/album")
            .route("/all", web::get().to(get_all_albums))
            .route("/unissued", web::get().to(get_all_unissued_albums))
            .route("/search/{album_name}", web::get().to(search_albums_by_name))
            .route("/play/{album_id}", web::get().to(play_album))
            .route("/issue/{album_id}", web::post().to(issue_album))
            .route("/{album_id}", web::get().to(get_album_by_id))
            .route("/{album_id}", web::delete().to(remove_album))
            .route("", web::post().to(create_album))
    );

    cfg.service(web::scope("/track")
        .route("/play/{track_id}", web::get().to(play_track))
        .route("/add/{album_id}", web::post().to(add_track_to_album))
        .route("/all", web::get().to(get_all_tracks))
    );

    cfg.service(
        web::scope("/favorite_albums")
            .route("/{album_id}", web::post().to(add_to_favorite_albums)) // need token
            .route("/{album_id}", web::delete().to(remove_from_favorite_albums)) // need token
            .route("", web::get().to(search_favorite_albums)) // need token
    );
    cfg.service(
        web::scope("/favorite_artists")
            .route("/{artist_id}", web::post().to(add_to_favorite_artists))
            .route(
                "/{artist_id}",
                web::delete().to(remove_from_favorite_artists)
            )
            .route("", web::get().to(search_favorite_artists))
    );

    cfg.service(
        web::scope("/comment")
            .route("/{album_id}", web::post().to(add_comment))
            .route("/{comment_id}", web::delete().to(delete_comment))
            .route("/{album_id}", web::get().to(get_comments_of_album))
    );
}
