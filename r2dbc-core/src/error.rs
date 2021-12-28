use thiserror::Error;
use url::Url;

/// R2DBC Errors

// TODO: do we need this error?
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum R2dbcErrors {
    #[error("Configuration error: `{0}`")]
    Configuration(String),

    #[error("General error: `{0}`")]
    General(String),

    #[error("Unsupported error: `{0}`")]
    Unsupported(String),

    #[error("URL parse error: `{0}`")]
    UrlParseError(#[from] url::ParseError),

    #[error("Unknown Database")]
    UnknownDatabase,
}

impl R2dbcErrors {

    // #[allow(dead_code)]
    // #[inline]
    // pub(crate) fn config(err: impl StdError + Send + Sync + 'static) -> Self {
    //     Error::Configuration(err.into())
    // }

    #[allow(dead_code)]
    #[inline]
    pub fn config(err: String) -> Self {
        R2dbcErrors::Configuration(err)
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum R2dbcError {
    BadGrammar,
    General,
    NonTransient,
    NonTransientResource,
    PermissionDenied,
    Rollback,
    Timeout,
    Transient,
    TransientResource
}

// index out of bounds
// IllegalArgumentException
// NoSuchElementException

// `Error::InvalidColumnType` if the underlying SQLite column  type is not a valid type as a source for `T`.
// `Error::InvalidColumnIndex` if `idx` is outside the valid column range for this row.
// `Error::InvalidColumnName` if `idx` is not a valid column name for this row.
