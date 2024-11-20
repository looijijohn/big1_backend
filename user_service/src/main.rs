// src/main.rs
mod models;
use models::{User, Role, Claims};
use actix_web::{web, App, HttpServer, Responder, HttpResponse, http::header::Authorization};
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::{ClientOptions, TransactionOptions},
    Client, Collection, Database,
};
use std::error::Error;
use tokio::sync::Mutex;
use std::sync::Arc;
use dotenv::dotenv;
use std::env;
use serde_json::json;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("user_service");
    let collection = db.collection::<User>("users");

    let host = env::var("HOST").expect("HOST must be set");
    let port = env::var("PORT").expect("PORT must be set");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Arc::new(Mutex::new(collection.clone()))))
            .service(web::resource("/signup").route(web::post().to(signup)))
            .service(web::resource("/login").route(web::post().to(login)))
            .service(web::resource("/users/{identifier}").route(web::get().to(get_user)))
            .service(web::resource("/users/{identifier}").route(web::put().to(update_user)))
            .service(web::resource("/users/{identifier}").route(web::delete().to(delete_user)))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}

async fn signup(user: web::Json<User>, collection: web::Data<Arc<Mutex<Collection<User>>>>) -> impl Responder {
    let user = user.into_inner();
    match create_user(&collection, user).await {
        Ok(_) => HttpResponse::Ok().json(json!({"status": "success"})),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

async fn login(credentials: web::Json<User>, collection: web::Data<Arc<Mutex<Collection<User>>>>) -> impl Responder {
    let credentials = credentials.into_inner();
    match get_user_by_credentials(&collection, &credentials.username, &credentials.password).await {
        Ok(Some(user)) => {
            let claims = Claims::new(user.username.clone(), user.role.clone());
            match claims.encode() {
                Ok(token) => HttpResponse::Ok().json(json!({"token": token})),
                Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
            }
        }
        Ok(None) => HttpResponse::Unauthorized().body("Invalid credentials"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

async fn get_user(identifier: web::Path<String>, collection: web::Data<Arc<Mutex<Collection<User>>>>, auth: Authorization<String>) -> impl Responder {
    match authenticate(&auth) {
        Ok(_claims) => match get_user_by_identifier(&collection, &identifier).await {
            Ok(Some(user)) => HttpResponse::Ok().json(user),
            Ok(None) => HttpResponse::NotFound().body("User not found"),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        },
        Err(e) => HttpResponse::Unauthorized().body(e.to_string()),
    }
}

async fn update_user(identifier: web::Path<String>, user: web::Json<User>, collection: web::Data<Arc<Mutex<Collection<User>>>>, auth: Authorization<String>) -> impl Responder {
    match authenticate(&auth) {
        Ok(_claims) => match update_user_by_identifier(&collection, &identifier, user.into_inner()).await {
            Ok(_) => HttpResponse::Ok().json(json!({"status": "success"})),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        },
        Err(e) => HttpResponse::Unauthorized().body(e.to_string()),
    }
}

async fn delete_user(identifier: web::Path<String>, collection: web::Data<Arc<Mutex<Collection<User>>>>, auth: Authorization<String>) -> impl Responder {
    match authenticate(&auth) {
        Ok(_claims) => match delete_user_by_identifier(&collection, &identifier).await {
            Ok(_) => HttpResponse::Ok().json(json!({"status": "success"})),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        },
        Err(e) => HttpResponse::Unauthorized().body(e.to_string()),
    }
}

async fn create_user(collection: &Arc<Mutex<Collection<User>>>, user: User) -> Result<(), Box<dyn Error>> {
    let session = collection.lock().await.client().start_session(None).await?;
    session.start_transaction(TransactionOptions::default()).await?;

    let mut collection = collection.lock().await;
    collection.insert_one_with_session(user, None, &mut session).await?;

    session.commit_transaction().await?;
    Ok(())
}

async fn get_user_by_credentials(collection: &Arc<Mutex<Collection<User>>>, username: &str, password: &str) -> Result<Option<User>, Box<dyn Error>> {
    let session = collection.lock().await.client().start_session(None).await?;
    session.start_transaction(TransactionOptions::default()).await?;

    let mut collection = collection.lock().await;
    let filter = doc! {
        "$or": [
            { "username": username, "password": password },
            { "email": username, "password": password }
        ]
    };
    let user = collection.find_one_with_session(filter, None, &mut session).await?;

    session.commit_transaction().await?;
    Ok(user)
}

async fn get_user_by_identifier(collection: &Arc<Mutex<Collection<User>>>, identifier: &str) -> Result<Option<User>, Box<dyn Error>> {
    let session = collection.lock().await.client().start_session(None).await?;
    session.start_transaction(TransactionOptions::default()).await?;

    let mut collection = collection.lock().await;
    let filter = doc! {
        "$or": [
            { "username": identifier },
            { "email": identifier }
        ]
    };
    let user = collection.find_one_with_session(filter, None, &mut session).await?;

    session.commit_transaction().await?;
    Ok(user)
}

async fn update_user_by_identifier(collection: &Arc<Mutex<Collection<User>>>, identifier: &str, user: User) -> Result<(), Box<dyn Error>> {
    let session = collection.lock().await.client().start_session(None).await?;
    session.start_transaction(TransactionOptions::default()).await?;

    let mut collection = collection.lock().await;
    let filter = doc! {
        "$or": [
            { "username": identifier },
            { "email": identifier }
        ]
    };
    let update = doc! { "$set": bson::to_bson(&user)? };
    collection.update_one_with_session(filter, update, None, &mut session).await?;

    session.commit_transaction().await?;
    Ok(())
}

async fn delete_user_by_identifier(collection: &Arc<Mutex<Collection<User>>>, identifier: &str) -> Result<(), Box<dyn Error>> {
    let session = collection.lock().await.client().start_session(None).await?;
    session.start_transaction(TransactionOptions::default()).await?;

    let mut collection = collection.lock().await;
    let filter = doc! {
        "$or": [
            { "username": identifier },
            { "email": identifier }
        ]
    };
    collection.delete_one_with_session(filter, None, &mut session).await?;

    session.commit_transaction().await?;
    Ok(())
}

fn authenticate(auth: &Authorization<String>) -> Result<Claims, Box<dyn Error>> {
    let token = auth.as_str().split("Bearer ").collect::<Vec<&str>>()[1];
    Claims::decode(token).map_err(|e| e.into())
}