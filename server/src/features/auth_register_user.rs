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
    pub async fn new(mongo_client: mongodb::Client) -> Result::<Self, self::errors::InitializationError> {
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
            .await?;

        Ok(Self { mongo_client })
    }

    pub async fn run(&self, email: String, password: String) -> Result<(), self::errors::FeatureError> {
        let password = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;

        self.mongo_client
            .database(DATABASE)
            .collection(COLLECTION)
            .insert_one(User { email, password }, None)
            .await?;

        Ok(())
    }
}

pub mod errors {
    #[derive(Debug, Clone)]
    pub struct InitializationError {
        message: String,
    }

    impl std::fmt::Display for InitializationError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Cannot initialize feature: {}", self.message)
        }
    }

    impl From<mongodb::error::Error> for InitializationError {
        fn from(value: mongodb::error::Error) -> Self {
            Self { message: value.to_string() }
        }
    }

    #[derive(Debug, Clone)]
    pub(crate) struct FeatureError {
        message: String,
    }

    impl std::fmt::Display for FeatureError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Cannot run feature: {}", self.message)
        }
    }

    impl From<bcrypt::BcryptError> for FeatureError {
        fn from(value: bcrypt::BcryptError) -> Self {
            Self::new(value.to_string())
        }
    }

    impl From<mongodb::error::Error> for FeatureError {
        fn from(value: mongodb::error::Error) -> Self {
            Self { message: value.to_string() }
        }
    }
}
