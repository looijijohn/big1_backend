use mongodb::{Client, options::ClientOptions};
use dotenv::dotenv;
use std::env;

pub async fn connect() -> Client {
    dotenv().ok();
    let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI must be set");
    let client_options = ClientOptions::parse(&mongo_uri).await.expect("Failed to parse options");
    Client::with_options(client_options).expect("Failed to connect to MongoDB")
}