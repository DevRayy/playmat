use serde::{Deserialize, Serialize};

const DATABASE: &str = "auth";
const COLLECTION: &str = "users";

#[derive(Serialize, Deserialize)]
struct User {
    email: String,
    password: String,
}


pub(crate) struct Users {
    client: mongodb::Client,
}

impl Users {
    pub fn new(client: mongodb::Client) -> Self {
        Self {
            client,
        }
    }

    pub async fn create(&self, email: String, password: String) -> Result<(), mongodb::error::Error> {
        let collection = self.client.database(DATABASE).collection(COLLECTION);

        collection.insert_one(User{
            email,
            password,
        }, None).await?;

        Ok(())
    }
}