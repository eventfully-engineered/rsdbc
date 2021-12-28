use thiserror::Error;
use r2dbc_core::error::R2dbcErrors;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum SqliteR2dbcError {
    // #[error("Configuration error: `{0}`")]
    // Configuration(String),
    //
    // #[error("General error: `{0}`")]
    // General(String),
    //
    // #[error("Unsupported error: `{0}`")]
    // Unsupported(String),
    //
    // #[error("URL parse error: `{0}`")]
    // UrlParseError(#[from] url::ParseError),
    //
    // #[error("Unknown Database")]
    // UnknownDatabase,

    #[error("Invalid Protocol: `{0}`")]
    InvalidProtocol(String),
}

impl From<SqliteR2dbcError> for R2dbcErrors {
    fn from(err: SqliteR2dbcError) -> R2dbcErrors {
        return match err {
            SqliteR2dbcError::InvalidProtocol(s) => {
                R2dbcErrors::Unsupported(s)
            }
        }
    }
}
