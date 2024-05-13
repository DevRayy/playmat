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
        Self { mongo_client }
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
pub enum Error {
    Unknown(String),
    HashingError(String),
    DuplicateEmail,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown(value) => write!(f, "Unknown error: {}", value),
            Self::HashingError(value) => write!(f, "Hashing error: {}", value),
            Self::DuplicateEmail => write!(f, "Duplicate email"),
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
        match value.kind.as_ref() {
            mongodb::error::ErrorKind::Write(write_error) => {
                match write_error {
                    mongodb::error::WriteFailure::WriteError(write_error) => {
                        if write_error.code == 11000 {
                            Self::DuplicateEmail
                        } else {
                            Self::Unknown(write_error.message.clone())
                        }
                    }
                    _ => Self::Unknown(format!("{:?}", write_error)),
                }
            }
            _ => Self::Unknown(value.to_string()),
        }
    }
}