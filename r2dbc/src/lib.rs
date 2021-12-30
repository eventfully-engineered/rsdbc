use std::collections::HashMap;
use std::error::Error as StdError;
use std::io;
use std::io::Read;
use std::str::FromStr;
use std::time::Duration;
use futures::future::BoxFuture;
use url::Url;
use r2dbc_core::error::R2dbcErrors;
use r2dbc_core::{OptionValue, Result};
use r2dbc_core::connection::{Connection, ConnectionFactory, ConnectionFactoryOptions, ConnectionFactoryProvider};
use r2dbc_sqlite::connection::SqliteConnectionFactory;
use r2dbc_postgres::PostgresqlConnectionFactory;

pub struct ConnectionFactories;

// TODO: use Any? https://doc.rust-lang.org/std/any/index.html
impl ConnectionFactories {

    // /// Returns a [ConnectionFactory] from an available implementation, created from a Connection URL.
    // fn new<S: Into<String>, C: ?Sized>(url: S) -> Result<dyn ConnectionFactory<'static, Connection = C>> {
    //     let options = ConnectionFactoryOptions::parse(url)?;
    //     return Self::create(options);
    // }

    /// Returns a [ConnectionFactory] from an available implementation, created from a Connection URL.
    fn new(url: String) -> std::result::Result<Box<dyn ConnectionFactory>, Box<dyn StdError>> {
        let options = ConnectionFactoryOptions::parse(url)?;
        return Self::create(options);
    }

    fn from(options: ConnectionFactoryOptions) -> std::result::Result<Box<dyn ConnectionFactory>, Box<dyn StdError>> {
        return Self::create(options);
    }

    fn create(options: ConnectionFactoryOptions) -> std::result::Result<Box<dyn ConnectionFactory>, Box<dyn StdError>> {
        // TODO: constant
        let driver = options.get_value("driver");
        if driver.is_none() {
            return Err(Box::new(R2dbcErrors::UnknownDatabase));
        }

        // TODO: simplify
        let driver_value = match driver.unwrap() {
            OptionValue::String(s) => {
                s
            }
            _ => {
                // TODO: return error
                ""
            }
        };

        let db = DB::from_str(driver_value);
        if db.is_err() {
            return Err(Box::new(db.err().unwrap()));
        }

        // TODO: fix unwraps
        let connection_factory: Option<Box<dyn ConnectionFactory>> = match db.unwrap() {
            DB::PostgreSQL => Some(Box::new(PostgresqlConnectionFactory::create(options).unwrap())),
            DB::SQLite => Some(Box::new(SqliteConnectionFactory::create(options).unwrap())),
            _ => None
        };

        // if connection_factory.is_none() {
        //     return Err(R2dbcErrors::UnknownDatabase);
        // }

        let cf = connection_factory.unwrap();

        Ok(cf)
    }
}




// TODO: call this Database? Engine? Backend?
pub enum DB {
    MySQL,
    PostgreSQL,
    SQLite,
    SQLServer,
    // Oracle
    MariaDB,
}

impl FromStr for DB {
    type Err = R2dbcErrors;

    fn from_str(input: &str) -> Result<DB> {
        match input {
            "postgresql" | "postgres"  => Ok(DB::PostgreSQL),
            "mysql"  => Ok(DB::MySQL),
            "mariadb" => Ok(DB::MariaDB),
            "sqlserver" | "mssql" => Ok(DB::SQLServer),
            "sqlite" => Ok(DB::SQLite),
            _      => Err(R2dbcErrors::UnknownDatabase),
        }
    }
}




#[cfg(test)]
mod tests {
    use crate::ConnectionFactoryOptions;

    #[test]
    fn parse() {
        let result = ConnectionFactoryOptions::parse("postgres://postgres:password@localhost/test");
        assert!(result.is_ok());

        println!("{:?}", result.unwrap());
    }
}

