use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct DbModel {
    email: String,
    password: String,
}

pub struct Query {
    pub email: String,
    pub password: String,
}

pub struct Feature {
    mongo_client: mongodb::Client,
}

impl Feature {
    pub async fn new(mongo_client: mongodb::Client) -> Self {
        Self { mongo_client }
    }

    pub async fn run(&self, data: Query) -> Result<(), Error> {
        validate_email(&data.email)?;

        let hashed_password = bcrypt::hash(data.password, bcrypt::DEFAULT_COST)?;

        self.mongo_client
            .database("auth")
            .collection("users")
            .insert_one(DbModel { email: data.email, password: hashed_password }, None)
            .await?;

        Ok(())
    }
}

fn validate_email(email: &str) -> Result<(), Error> {
    if email.contains('@') {
        Ok(())
    } else {
        Err(Error::InvalidEmail)
    }
}

#[derive(Debug, Clone)]
pub enum Error {
    Unknown(String),
    Hashing(String),
    DuplicateEmail,
    InvalidEmail,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown(value) => write!(f, "Unknown error: {}", value),
            Self::Hashing(value) => write!(f, "Hashing error: {}", value),
            Self::DuplicateEmail => write!(f, "Duplicate email"),
            Self::InvalidEmail => write!(f, "Invalid email"),
        }
    }
}

impl From<bcrypt::BcryptError> for Error {
    fn from(value: bcrypt::BcryptError) -> Self {
        Self::Hashing(value.to_string())
    }
}

impl From<mongodb::error::Error> for Error {
    fn from(value: mongodb::error::Error) -> Self {
        match value.kind.as_ref() {
            mongodb::error::ErrorKind::Write(write_error) => match write_error {
                mongodb::error::WriteFailure::WriteError(write_error) => {
                    if write_error.code == 11000 {
                        Self::DuplicateEmail
                    } else {
                        Self::Unknown(write_error.message.clone())
                    }
                }
                _ => Self::Unknown(format!("{:?}", write_error)),
            },
            _ => Self::Unknown(value.to_string()),
        }
    }
}
