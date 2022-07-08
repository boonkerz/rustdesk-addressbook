use actix_web::web::{post, resource, scope, ServiceConfig};
use log::info;

use crate::api;

pub fn config_routes(cfg: &mut ServiceConfig) {
    info!("Configurating the routes...");
    cfg.service(
        scope("/api")
            .service(resource("/register").route(post().to(api::user::register)))
            .service(resource("/login").route(post().to(api::user::login)))
            .service(resource("/currentUser").route(post().to(api::user::current_user)))
            .service(resource("/ab/get").route(post().to(api::ab::get)))
            .service(resource("/ab").route(post().to(api::ab::post)))
    );
}


pub const IGNORE_ROUTES: [&str; 2] = ["/api/register", "/api/login"];