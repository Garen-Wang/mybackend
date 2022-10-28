extern crate diesel;

pub mod user;
pub mod healthcheck;

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
    let frontend_url = "localhost:8000";
    let backend_url = "localhost:7878";
    std::env::set_var("RUST_LOG", "actix_web=debug");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    env_logger::init();

    let manager = Manager::new(database_url);
    let pool = r2d2::Pool::builder().build(manager).expect("failed to build pool");

    let app_state = AppState { pool };

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(frontend_url)
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT]) // TODO: headers not enough
            .max_age(3600);

        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(app_state.clone()))
            .wrap(cors)
            .wrap(auth::Authorization)
            .configure(routes::routes)
    })
    .bind(backend_url)?
    .run()
    .await
}
