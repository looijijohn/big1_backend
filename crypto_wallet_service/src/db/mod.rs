use mongodb::{Client, Database};
use log::info;

pub async fn connect_to_mongodb(uri: &str, db_name: &str) -> Result<Database, mongodb::error::Error> {
    let client = Client::with_uri_str(uri).await?;
    info!("Successfully connected to MongoDB");
    Ok(client.database(db_name))
}