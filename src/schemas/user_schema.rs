use async_graphql::{Object, ID};

#[derive(Clone)]
pub struct User {
    pub id: ID,
    pub name: String,
    pub lastname: String,
}

#[Object]
impl User {
    async fn id(&self) -> &ID {
        &self.id
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn lastname(&self) -> &str {
        &self.lastname
    }
}