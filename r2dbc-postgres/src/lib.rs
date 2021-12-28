use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::time::Duration;
use futures_core::future::BoxFuture;
use postgres::config::SslMode;
use url::Url;
use r2dbc_core::connection::{Batch, Connection, ConnectionFactory, ConnectionFactoryMetadata, ConnectionMetadata, IsolationLevel, Statement, ValidationDepth};
use r2dbc_core::error::R2dbcErrors;
use r2dbc_core::Result;

pub struct  PostgresqlConnectionConfiguration {
    // application_name = "r2dbc-postgresql"
    pub auto_detect_extensions: bool, // true
    pub compatibility_mode: bool,
    pub connection_timeout: Duration,
    pub fetch_size: u64,
    pub force_binary: bool,
    pub host: String,
    pub options: HashMap<String, String>,
    pub password: String,
    pub port: u32, // default port
    pub prepared_statement_cache_queries: u32,
    pub schema: String,
    pub socket: String,
    pub ssl_cert: Url,
    // pub hostname_verifier: HostnameVerifier,
    pub ssl_key: Url,
    pub ssl_mode: SslMode, // TODO: expose own so that we can change internals if we need to
    pub ssl_password: String,
    pub ssl_root_cert: Url,
    pub statement_timeout: Duration,
    pub tcp_keep_alive: bool,
    pub tcp_no_delay: bool, // true
    pub username: String,

}

pub struct PostgresqlConnectionFactory {

}

pub struct PostgresqlConnection {

}

impl Connection for PostgresqlConnection {
    fn begin_transaction(&mut self) -> Result<()> {
        todo!()
    }

    fn close(&mut self) -> Result<()> {
        todo!()
    }

    fn commit_transaction(&mut self) {
        todo!()
    }

    fn create_batch(&mut self) -> Result<Box<dyn Batch>> {
        todo!()
    }

    fn create_savepoint(&mut self, name: &str) {
        todo!()
    }

    fn create_statement(&mut self, sql: &str) -> Result<Box<dyn Statement<'_> + '_>> {
        todo!()
    }

    fn is_auto_commit(&mut self) -> bool {
        todo!()
    }

    fn metadata(&mut self) -> Result<Box<dyn ConnectionMetadata>> {
        todo!()
    }

    fn transaction_isolation_level(&mut self) -> IsolationLevel {
        todo!()
    }

    fn release_savepoint(&mut self, name: &str) {
        todo!()
    }

    fn rollback_transaction(&mut self) {
        todo!()
    }

    fn rollback_transaction_to_savepoint(&mut self, name: String) {
        todo!()
    }

    fn auto_commit(&mut self, commit: bool) {
        todo!()
    }

    fn set_transaction_isolation_level(&mut self, isolation_level: IsolationLevel) {
        todo!()
    }

    fn validate(&mut self, depth: ValidationDepth) {
        todo!()
    }
}

impl ConnectionFactory for PostgresqlConnectionFactory {
    fn connect(&self) -> Pin<Box<dyn Future<Output = Result<Box<(dyn Connection + 'static)>>> + Send>> {
        todo!()
    }
    // fn connect(&self) -> BoxFuture<'_, Result<Box<PostgresqlConnection>>> {
    //     todo!()
    // }


    // TODO: change to Postgres Connection factory metadata?
    fn get_metadata(&self) -> Box<dyn ConnectionFactoryMetadata> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
