mod config;
mod db;
mod models;
mod routes;
mod services;
mod middleware;
mod utils;
mod errors;
mod jwt; // Declare the jwt module

use actix_web::{web, App, HttpServer, middleware::Logger};
use mongodb::Database;
use dotenv::dotenv;
use std::env;
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv().ok();
    let (mongo_uri, database_name, jwt_secret) = config::load_config();

    // Initialize logger
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    // Connect to MongoDB
    let db_result = db::connect_to_mongodb(&mongo_uri, &database_name).await;

    // Log connection status
    match &db_result {
        Ok(_) => info!("Successfully connected to MongoDB"),
        Err(e) => info!("Failed to connect to MongoDB: {}", e),
    }

    let db = db_result.unwrap();

    // Initialize the Actix web server
    HttpServer::new(move || {
        App::new()
            .data(db.clone())
            .data(jwt_secret.clone())
            .wrap(Logger::default())
            .service(
                web::resource("/wallets")
                    .route(web::post().to(routes::create_wallet_route))
                    .route(web::get().to(routes::get_wallet_route))
                    .route(web::put().to(routes::update_balance_route))
                    .route(web::delete().to(routes::delete_wallet_route))
            )
            .service(
                web::resource("/create_jwt")
                    .route(web::post().to(jwt::create_jwt_route))
            )
            
    })
    .bind("127.0.0.1:7070")?
    .run()
    .await
}