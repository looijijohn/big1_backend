use actix_web::web;
use crate::handlers::{register, login, get_users};
 
use actix_web::{  App, HttpServer, Responder};

use apistos::{OpenApi   };  // Import OpenApi from apistos

use serde::{Serialize, Deserialize};


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/users")
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
            .route("", web::get().to(get_users))
           
    );
}