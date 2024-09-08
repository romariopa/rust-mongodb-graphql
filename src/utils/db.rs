use mongodb::{Client, Database};
use std::sync::Arc;
use std::env;
use dotenv::dotenv;

#[derive(Clone)]
#[allow(dead_code)]
pub struct DatabaseConnection {
    client: Arc<Client>,
    database: Arc<Database>,
}

impl DatabaseConnection {
    pub async fn new() -> Self {
        dotenv().ok();
        let mongo_uri = env::var("MONGODB_STRING").expect("MONGODB_STRING must be set");
        let client = Client::with_uri_str(&mongo_uri.to_string()).await.unwrap();
        let db_name = env::var("DATABASE_NAME").expect("MONGODB_DB_NAME must be set");
        let database = client.database(&db_name.to_string());

        DatabaseConnection {
            client: Arc::new(client),
            database: Arc::new(database)
        }
    }

    pub fn get_database(&self) -> &Database {
        &self.database
    }
}