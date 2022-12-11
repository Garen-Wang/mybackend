use actix_web::web;

use crate::{
    album::handler::search_albums_by_name, artist::handler::search_artists_by_name,
    favorite::handler::{search_favorites, add_to_favorites, remove_from_favorites}, healthcheck, user,
};

pub fn healthcheck_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/healthcheck").route("", web::get().to(healthcheck::handler::index)));
}

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("login", web::post().to(user::handler::login))
            .route("register", web::post().to(user::handler::register))
    );
}

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .route("", web::get().to(user::handler::get_myself)) // need token
            .route("", web::put().to(user::handler::update)) // need token
    );
}

pub fn artist_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/artist")
        .route(
            "/search/{artist_name}",
            web::get().to(search_artists_by_name)
        )
    );
}

pub fn album_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/album")
        .route("/search/{album_name}", web::get().to(search_albums_by_name))
    );
}

pub fn favorite_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/favorite")
        .route("/search", web::get().to(search_favorites)) // need token
        .route("/add/{album_id}", web::get().to(add_to_favorites)) // need token
        .route("/remove/{album_id}", web::get().to(remove_from_favorites)) // need token
    );
}
