use std::collections::HashMap;
use std::error::Error as StdError;
use std::io;
use std::io::Read;
use std::str::FromStr;
use std::time::Duration;
use futures::future::BoxFuture;
use url::Url;
use rsdbc_core::error::RsdbcErrors;
use rsdbc_core::{OptionValue, Result};
use rsdbc_core::connection::{Connection, ConnectionFactory, ConnectionFactoryOptions, ConnectionFactoryProvider};
use rsdbc_sqlite::connection::SqliteConnectionFactory;
use rsdbc_postgres::PostgresqlConnectionFactory;

pub struct ConnectionFactories;

// TODO: use Any? https://doc.rust-lang.org/std/any/index.html
impl ConnectionFactories {

    // /// Returns a [ConnectionFactory] from an available implementation, created from a Connection URL.
    // fn new<S: Into<String>, C: ?Sized>(url: S) -> Result<dyn ConnectionFactory<'static, Connection = C>> {
    //     let options = ConnectionFactoryOptions::parse(url)?;
    //     return Self::create(options);
    // }

    /// Returns a [ConnectionFactory] from an available implementation, created from a Connection URL.
    fn new(url: &str) -> std::result::Result<Box<dyn ConnectionFactory>, Box<dyn StdError>> {
        let options = ConnectionFactoryOptions::parse(url)?;
        return Self::create(options);
    }

    fn from(options: ConnectionFactoryOptions) -> std::result::Result<Box<dyn ConnectionFactory>, Box<dyn StdError>> {
        return Self::create(options);
    }

    fn create(options: ConnectionFactoryOptions) -> std::result::Result<Box<dyn ConnectionFactory>, Box<dyn StdError>> {
        // TODO: constant
        println!("{:?}", options);
        let driver = options.get_value("driver");
        if driver.is_none() {
            return Err(Box::new(RsdbcErrors::UnknownDatabase));
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
        //     return Err(RsdbcErrors::UnknownDatabase);
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
    type Err = RsdbcErrors;

    fn from_str(input: &str) -> Result<DB> {
        match input {
            "postgresql" | "postgres"  => Ok(DB::PostgreSQL),
            "mysql"  => Ok(DB::MySQL),
            "mariadb" => Ok(DB::MariaDB),
            "sqlserver" | "mssql" => Ok(DB::SQLServer),
            "sqlite" => Ok(DB::SQLite),
            _      => Err(RsdbcErrors::UnknownDatabase),
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::{ConnectionFactories, ConnectionFactoryOptions};

    #[test]
    fn connection_factories_new_should_create_connection_factory_for_valid_connection_string() {
        let connection_factory_result = ConnectionFactories::new("postgres://admin:password@localhost/test");
        assert!(connection_factory_result.is_ok());
    }

    #[test]
    fn connection_factories_new_should_create_connection_factory_for_valid_options() {
        let options = ConnectionFactoryOptions::parse("postgres://admin:password@localhost/test").unwrap();
        let connection_factory_result = ConnectionFactories::from(options);
        assert!(connection_factory_result.is_ok());
    }

    // TODO: invalid url should return error

    // TODO: unsupported db should return unknown db error
}

