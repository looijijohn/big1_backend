mod db;
mod models;
mod routes;
mod handlers;
mod services;

use actix_web::{App, HttpServer, web};
use crate::db::connect;
use crate::routes::config;
    
 



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = connect().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .configure(config)
         
    })
    


    .bind("127.0.0.1:9090")?
    .run()
    .await
}