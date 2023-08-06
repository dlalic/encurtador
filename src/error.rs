use crate::error::Error::{BadRequest, InternalServerError};

use diesel::result::DatabaseErrorKind;
use diesel::result::Error::{DatabaseError, NotFound};
use diesel_migrations::MigrationError;
use gotham_restful::ResourceError;
use log::error;
use serde::ser::StdError;
use validator::{ValidationError, ValidationErrors};

#[derive(Debug, PartialEq, ResourceError)]
pub enum Error {
    #[status(BAD_REQUEST)]
    #[display("{0}")]
    BadRequest(String),
    #[status(FORBIDDEN)]
    #[display("Forbidden")]
    Forbidden,
    #[status(INTERNAL_SERVER_ERROR)]
    #[display("Internal Server Error")]
    InternalServerError,
}

impl From<diesel::result::Error> for Error {
    fn from(error: diesel::result::Error) -> Self {
        error!("Diesel: {error:?}");
        match error {
            NotFound => BadRequest("Not found".to_string()),
            DatabaseError(DatabaseErrorKind::UniqueViolation, ..) => {
                BadRequest("Already exists".to_string())
            }
            _ => InternalServerError,
        }
    }
}

impl From<Error> for ValidationError {
    fn from(error: Error) -> Self {
        error!("Validation: {error:?}");
        ValidationError::new("invalid")
    }
}

impl From<ValidationErrors> for Error {
    fn from(error: ValidationErrors) -> Self {
        error!("Validation: {error:?}");
        BadRequest(error.to_string())
    }
}

impl From<MigrationError> for Error {
    fn from(error: MigrationError) -> Self {
        error!("Migrations: {error:?}");
        InternalServerError
    }
}

impl From<Box<dyn StdError + Send + Sync>> for Error {
    fn from(error: Box<dyn StdError + Send + Sync>) -> Self {
        error!("Migrations: {error:?}");
        InternalServerError
    }
}
