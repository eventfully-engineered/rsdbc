use futures::future::BoxFuture;
use std::time::Duration;
use url::Url;
use r2dbc_core::connection::{Batch, Connection, ConnectionFactory, ConnectionFactoryMetadata, ConnectionFactoryOptions, ConnectionFactoryProvider, ConnectionMetadata, IsolationLevel, SslMode, Statement, ValidationDepth};
use r2dbc_core::Result;


pub struct MySqlConnectionConfiguration {
    pub domain: String,
    pub port: i32,
    pub ssl: MySqlSslConfiguration,
    pub tcp_keep_alive: bool,
    pub tcp_no_delay: bool, // true
    pub connection_timeout: Duration,
    pub socket_timeout: Duration,
    pub username: String,
    pub password: String,
    // pub prefer_prepared_statements: String,
    pub query_cache_size: i32,
    pub prepare_cache_size: i32,

}

pub struct MySqlSslConfiguration {
    pub ssl_mode: SslMode, // TODO: expose own so that we can change internals if we need to
    pub tls_version: Vec<String>,
    // pub hostname_verifier: HostnameVerifier,
    pub ssl_cert: Url,
    pub ssl_key: Url,
    pub ssl_password: String,
    pub ssl_root_cert: Url, // sslCa?
}

pub struct MySqlConnection;

impl Connection for MySqlConnection {
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

    fn validate(&mut self, depth: ValidationDepth) -> bool {
        todo!()
    }
}

pub struct MySqlConnectionFactory;
impl ConnectionFactory for MySqlConnectionFactory {
    fn connect(&self) -> BoxFuture<'_, Result<Box<dyn Connection>>> {
        todo!()
    }

    fn get_metadata(&self) -> Box<dyn ConnectionFactoryMetadata> {
        todo!()
    }
}

impl ConnectionFactoryProvider for MySqlConnectionFactory {
    type C = MySqlConnectionFactory;

    fn create(options: ConnectionFactoryOptions) -> Result<Self::C> {
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
