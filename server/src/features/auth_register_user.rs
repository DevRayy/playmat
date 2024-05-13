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
    pub async fn new(mongo_client: mongodb::Client) -> Result::<Self, InitializationError> {
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

    pub async fn run(&self, email: String, password: String) -> Result<(), Error> {
        let password = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;

        self.mongo_client
            .database(DATABASE)
            .collection(COLLECTION)
            .insert_one(User { email, password }, None)
            .await?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum InitializationError {
    Mongo(String),
}

impl From<mongodb::error::Error> for InitializationError {
    fn from(value: mongodb::error::Error) -> Self {
        Self::Mongo(value.to_string())
    }
}

#[derive(Debug, Clone)]
pub enum Error {
    Unknown(String),
    HashingError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown(value) => write!(f, "Unknown error: {}", value),
            Self::HashingError(value) => write!(f, "Hashing error: {}", value),
        }
    }
}

impl From<bcrypt::BcryptError> for Error {
    fn from(value: bcrypt::BcryptError) -> Self {
        Self::HashingError(value.to_string())
    }
}

impl From<mongodb::error::Error> for Error {
    fn from(value: mongodb::error::Error) -> Self {
        Self::Unknown(value.to_string())
    }
}