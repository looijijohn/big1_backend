use mongodb::{
    error::Error,
    options::FindOptions,
    Client, Collection,
};
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use futures::stream::StreamExt; // Correct stream trait for next()

pub async fn find_paginated<T>(
    client: &Client,
    db_name: &str,
    collection_name: &str,
    page: u64,
    limit: u64,
) -> Result<Vec<T>, Error>
where
    T: Serialize + DeserializeOwned,
{
    let collection: Collection<T> = client.database(db_name).collection(collection_name);
    let skip = (page - 1) * limit;

    // Set `skip` and `limit` based on `page` and `limit` parameters
    let options = FindOptions::builder()
        .skip(Some(skip))
        .limit(Some(limit as i64))
        .build();

    // Perform the query and collect the results as a Vec<T>
    let mut cursor = collection.find(None, options).await?;
    let mut results = Vec::new();

    // Iterate over the cursor as a stream of documents
    while let Some(doc) = cursor.next().await {
        match doc {
            Ok(d) => results.push(d),  // Collect valid documents
            Err(e) => return Err(e),   // Return error if cursor returns an error
        }
    }

    Ok(results)
}
