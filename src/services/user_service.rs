use mongodb::bson::doc;
use mongodb::error::Error;
use crate::utils::db::DatabaseConnection;

pub async fn get_user_by_name(db: &DatabaseConnection, name: &str) -> Result<Option<mongodb::bson::Document>, Error> {
    let collection = db.get_database().collection("users");
    let user = collection.find_one(doc! { "name": name }).await?;
    Ok(user)
}