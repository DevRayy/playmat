use std::error::Error;

use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

const DATABASE: &str = "auth";
const COLLECTION: &str = "users";

#[derive(Serialize, Deserialize)]
struct User {
    email: String,
    password: String,
}

pub(crate) struct Feature {
    mongo_client: mongodb::Client,
}

impl Feature {
    pub async fn new(mongo_client: mongodb::Client) -> Self {
        mongo_client
            .database(DATABASE)
            .collection::<User>(COLLECTION)
            .create_index(
                mongodb::IndexModel::builder()
                    .keys(doc! { "email": 1 })
                    .options(
                        mongodb::options::IndexOptions::builder()
                            .unique(true)
                            .build(),
                    )
                    .build(),
                None,
            )
            .await
            .unwrap();

        Self { mongo_client }
    }

    pub async fn run(&self, email: String, password: String) -> Result<(), Box<dyn Error>> {
        let password = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;

        self.mongo_client
            .database(DATABASE)
            .collection(COLLECTION)
            .insert_one(User { email, password }, None)
            .await?;

        Ok(())
    }
}
