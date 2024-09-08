use async_graphql::{Object, ID};
use crate::schemas::user_schema::User;
use crate::services::user_service::get_user_by_name;
use crate::utils::db::DatabaseConnection;
use mongodb::error::Error;

#[derive(Clone)]
pub struct Query {
    pub db: DatabaseConnection,
}

#[Object]
impl Query {
    async fn user(&self, name: String) -> Result<Option<User>, Error> {
        if let Some(user_doc) = get_user_by_name(&self.db, &name).await? {
            let user = User {
                id: ID::from(user_doc.get_object_id("_id").map_or_else(|_| "".to_string(), |id| id.to_hex())),
                name: user_doc.get_str("name").unwrap_or("").to_string(),
                lastname: user_doc.get_str("last_name").unwrap_or("").to_string(),
            };
            Ok(Some(user))
        } else {
            Ok(None)
        }
    }
}