pub fn load_config() -> (String, String, String) {
    dotenv::dotenv().ok();
    let mongo_uri = std::env::var("MONGO_URI").expect("MONGO_URI must be set");
    let database_name = std::env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    (mongo_uri, database_name, jwt_secret)
}