use mongodb::{Database, bson::doc};
use crate::models::Wallet;
use crate::errors::AppError;

pub async fn create_wallet(db: Database, user_id: String, currency: String) -> Result<Wallet, AppError> {
    let wallet = Wallet {
        user_id: user_id.clone(),
        balance: 0.0,
        currency,
    };
    let collection = db.collection::<Wallet>("wallets");
    collection.insert_one(wallet.clone(), None).await?;
    Ok(wallet)
}

pub async fn get_wallet(db: Database, user_id: String) -> Result<Option<Wallet>, AppError> {
    let collection = db.collection::<Wallet>("wallets");
    collection.find_one(doc! {"user_id": user_id}, None).await.map_err(AppError::from)
}

pub async fn update_balance(db: Database, user_id: String, amount: f64) -> Result<(), AppError> {
    let collection = db.collection::<Wallet>("wallets");
    collection.update_one(
        doc! {"user_id": user_id},
        doc! {"$inc": {"balance": amount}},
        None,
    ).await.map_err(AppError::from)?;
    Ok(())
}

pub async fn delete_wallet(db: Database, user_id: String) -> Result<(), AppError> {
    let collection = db.collection::<Wallet>("wallets");
    collection.delete_one(doc! {"user_id": user_id}, None).await.map_err(AppError::from)?;
    Ok(())
}