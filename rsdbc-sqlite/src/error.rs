use thiserror::Error;
use rsdbc_core::error::RsdbcErrors;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum SqliteRsdbcError {
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

impl From<SqliteRsdbcError> for RsdbcErrors {
    fn from(err: SqliteRsdbcError) -> RsdbcErrors {
        return match err {
            SqliteRsdbcError::InvalidProtocol(s) => {
                RsdbcErrors::Unsupported(s)
            }
        }
    }
}
