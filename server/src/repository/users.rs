use mongodb::bson::doc;
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
    pub async fn new(client: mongodb::Client) -> Self {
        let s = Self { client };
        s.create_indices().await.unwrap();

        s
    }

    pub async fn create(
        &self,
        email: String,
        password: String,
    ) -> Result<(), mongodb::error::Error> {
        let collection = self.client.database(DATABASE).collection::<User>(COLLECTION);

        collection
            .insert_one(User { email, password }, None)
            .await?;

        Ok(())
    }

    async fn create_indices(&self) -> Result<(), mongodb::error::Error> {
        let collection = self.client.database(DATABASE).collection::<User>(COLLECTION);

        collection.create_index(
            mongodb::IndexModel::builder()
                .keys(doc! { "email": 1 })
                .options(mongodb::options::IndexOptions::builder().unique(true).build())
                .build(),
            None,
        ).await?;

        Ok(())
    }
}
