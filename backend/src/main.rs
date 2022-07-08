extern crate dotenv;

mod api;
mod repository;
mod service;

use api::routes::config_routes;
use dotenv::dotenv;
use repository::{user::UserRepository, ab::AbRepository};
use service::{auth_new::{Auth}};
use std::env;
use actix_web::{HttpServer, App, web::Data, middleware};
use migration::{Migrator, MigratorTrait};

#[derive(Debug, Clone)]
pub struct AppState {
    user_repository: UserRepository,
    ab_repository: AbRepository
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env::set_var("RUST_LOG", "debug");
    env::set_var("RUST_BACKTRACE", "1");

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    env_logger::init();
    let conn = sea_orm::Database::connect(&db_url).await.unwrap();
    Migrator::up(&conn, None).await.unwrap();
    
    let state = AppState { user_repository: UserRepository { db_conn: conn.clone() }, ab_repository: AbRepository { db_conn: conn.clone() }};

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(Data::new(state.clone()))
            .wrap(Auth)
            .configure(config_routes)
            

    })
    .bind(("0.0.0.0", 21114))?
    .run()
    .await
}
