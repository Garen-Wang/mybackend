use actix_web::web;

use crate::{healthcheck, user};

pub fn healthcheck_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/healthcheck").route("", web::get().to(healthcheck::handler::index)));
}

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("login", web::post().to(user::handler::login))
            .route("register", web::post().to(user::handler::register)),
    );
}

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .route("", web::get().to(user::handler::get_myself))
            .route("", web::put().to(user::handler::update)),
    );
}
