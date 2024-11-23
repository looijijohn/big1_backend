use actix_web::web;
use crate::handlers::{register, login, get_users};
use apistos::{openapi, OpenApi};




pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/users")
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
            .route("", web::get().to(get_users))
            .service(OpenApi::new("/openapi"))
    );
}