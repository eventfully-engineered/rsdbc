// From SQLx - https://github.com/launchbadge/sqlx/blob/master/sqlx-core/src/sqlite/options/parse.rs
// https://www.sqlite.org/uri.html

use std::borrow::Cow;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use crate::options::SqliteConnectOptions;
use percent_encoding::percent_decode_str;
use rsdbc_core::error::RsdbcErrors;

impl FromStr for SqliteConnectOptions {
    type Err = RsdbcErrors;

    fn from_str(mut uri: &str) -> Result<Self, Self::Err> {
        let mut options = Self::new();

        // remove scheme from the URI
        uri = uri
            .trim_start_matches("sqlite://")
            .trim_start_matches("sqlite:");

        let mut database_and_params = uri.splitn(2, '?');

        let database = database_and_params.next().unwrap_or_default();

        if database == ":memory:" {
            options.in_memory = true;
            // setting shared_cache to true. See https://www.sqlite.org/sharedcache.html
            options.shared_cache = true;
            options.filename = Cow::Owned(PathBuf::from(database));
        } else {
            // % decode to allow for `?` or `#` in the filename
            options.filename = Cow::Owned(
                Path::new(
                    &*percent_decode_str(database)
                        .decode_utf8()
                        .map_err(|e| RsdbcErrors::config(e.to_string()))?,
                ).to_path_buf(),
            );
        }

        if let Some(params) = database_and_params.next() {
            for (key, value) in url::form_urlencoded::parse(params.as_bytes()) {
                match &*key {
                    // The mode query parameter determines if the new database is opened read-only,
                    // read-write, read-write and created if it does not exist, or that the
                    // database is a pure in-memory database that never interacts with disk,
                    // respectively.
                    "mode" => {
                        match &*value {
                            "ro" => {
                                options.read_only = true;
                            }

                            // default
                            "rw" => {}

                            "rwc" => {
                                options.create_if_missing = true;
                            }

                            "memory" => {
                                options.in_memory = true;
                                options.shared_cache = true;
                            }

                            _ => {
                                return Err(RsdbcErrors::Configuration(
                                    format!("unknown value {:?} for `mode`", value).into(),
                                ));
                            }
                        }
                    }

                    // The cache query parameter specifies the cache behaviour across multiple
                    // connections to the same database within the process. A shared cache is
                    // essential for persisting data across connections to an in-memory database.
                    "cache" => match &*value {
                        "private" => {
                            options.shared_cache = false;
                        }

                        "shared" => {
                            options.shared_cache = true;
                        }

                        _ => {
                            return Err(RsdbcErrors::Configuration(
                                format!("unknown value {:?} for `cache`", value).into(),
                            ));
                        }
                    },

                    _ => {
                        return Err(RsdbcErrors::Configuration(
                            format!(
                                "unknown query parameter `{}` while parsing connection URI",
                                key
                            ).into(),
                        ));
                    }
                }
            }
        }

        Ok(options)
    }
}

#[test]
fn parse_in_memory() -> Result<(), RsdbcErrors> {
    let options: SqliteConnectOptions = "sqlite::memory:".parse()?;
    assert!(options.in_memory);
    assert!(options.shared_cache);

    let options: SqliteConnectOptions = "sqlite://?mode=memory".parse()?;
    assert!(options.in_memory);
    assert!(options.shared_cache);

    let options: SqliteConnectOptions = "sqlite://:memory:".parse()?;
    assert!(options.in_memory);
    assert!(options.shared_cache);

    let options: SqliteConnectOptions = "sqlite://?mode=memory&cache=private".parse()?;
    assert!(options.in_memory);
    assert!(!options.shared_cache);

    Ok(())
}

#[test]
fn parse_read_only() -> Result<(), RsdbcErrors> {
    let options: SqliteConnectOptions = "sqlite://a.db?mode=ro".parse()?;
    assert!(options.read_only);
    assert_eq!(&*options.filename.to_string_lossy(), "a.db");

    Ok(())
}

#[test]
fn parse_shared_in_memory() -> Result<(), RsdbcErrors> {
    let options: SqliteConnectOptions = "sqlite://a.db?cache=shared".parse()?;
    assert!(options.shared_cache);
    assert_eq!(&*options.filename.to_string_lossy(), "a.db");

    Ok(())
}

#[test]
fn from_str() -> Result<(), RsdbcErrors> {
    let options = SqliteConnectOptions::from_str("sqlite://a.db")?;
    assert_eq!(&*options.filename.to_string_lossy(), "a.db");
    assert!(!options.shared_cache);
    assert!(!options.in_memory);
    assert!(!options.read_only);
    assert!(!options.create_if_missing);

    Ok(())
}
