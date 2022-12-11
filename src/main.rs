extern crate diesel;

pub mod healthcheck;
pub mod user;
pub mod album;
pub mod artist;
pub mod track;
pub mod favorite;

pub mod token;
pub mod auth;
pub mod error;
pub mod routes;
pub mod schema;

use std::io;

use actix_cors::Cors;
use actix_web::{http, middleware::Logger, web, App, HttpServer};
use diesel::{r2d2::{self, ConnectionManager, PooledConnection}, PgConnection};
use dotenv::dotenv;
use error::AppError;

pub type Manager = ConnectionManager<PgConnection>;
pub type Pool = r2d2::Pool<Manager>;
pub type AppConn = PooledConnection<Manager>;

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool,
}

impl AppState {
    pub fn conn(&self) -> Result<AppConn, AppError> {
        let conn = self.pool.get()?;
        Ok(conn)
    }
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    // std::env::set_var("RUST_LOG", "actix_web=debug");
    std::env::set_var("RUST_LOG", "debug");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    env_logger::init();

    let manager = Manager::new(database_url);
    let pool = r2d2::Pool::builder().build(manager).expect("failed to build pool");

    let app_state = AppState { pool };

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "DELETE"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(app_state.clone()))
            .wrap(cors)
            .wrap(auth::Authorization)
            .configure(routes::healthcheck_routes)
            .configure(routes::auth_routes)
            .configure(routes::user_routes)
            .configure(routes::artist_routes)
            .configure(routes::album_routes)
    })
    .bind("localhost:7878")?
    .run()
    .await
}
