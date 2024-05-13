use std::fmt::{Display, Formatter};

use mongodb::bson::doc;

pub(crate) struct Feature {
    db: mongodb::Client,
}

impl Feature {
    pub fn new(db: mongodb::Client) -> Self {
        Self {
            db
        }
    }

    pub async fn run(&self) -> Result<(), Error> {
        self.db
            .database("auth")
            .collection::<Self>("users")
            .create_index(
                mongodb::IndexModel::builder()
                    .keys(doc! { "email": 1 })
                    .options(
                        mongodb::options::IndexOptions::builder().unique(true).build(),
                    )
                    .build(),
                None,
            )
            .await?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum Error {
    IndexCreationError(String),
}

impl From<mongodb::error::Error> for Error {
    fn from(value: mongodb::error::Error) -> Self {
        Self::IndexCreationError(value.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IndexCreationError(value) => write!(f, "Index creation error: {}", value),
        }
    }
}

impl std::error::Error for Error {}