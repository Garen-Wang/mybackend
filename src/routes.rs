use actix_web::web;

use crate::{healthcheck, user};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/healthcheck").route("", web::get().to(healthcheck::api::index))
    ).service(
        web::scope("/auth")
        .route("login", web::post().to(user::api::login))
        .route("register", web::post().to(user::api::register))
    ).service(
        web::scope("/user")
        .route("", web::get().to(user::api::get_myself))
        .route("", web::put().to(user::api::update))
    );
}
