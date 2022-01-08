mod ssl_mode;

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::time::Duration;
use futures::future::BoxFuture;
// use postgres::{Client, NoTls};
// use postgres::config::SslMode;
use tokio_postgres::{Client, NoTls};
use tokio_postgres::config::SslMode;
use native_tls::{Certificate, TlsConnector};
use postgres_native_tls::MakeTlsConnector;
use std::fs;

use tracing_subscriber::fmt::time;
use url::Url;
use rsdbc_core::connection::{Batch, Connection, ConnectionFactory, ConnectionFactoryMetadata, ConnectionFactoryOptions, ConnectionFactoryProvider, ConnectionMetadata, IsolationLevel, Statement, ValidationDepth};
use rsdbc_core::error::RsdbcErrors;
use rsdbc_core::{OptionValue, Result, TransactionDefinition};

// TODO: should this take raw string?
pub struct  PostgresqlConnectionConfiguration {
    // application_name = "rsdbc-postgresql"
    pub application_name: String,
    pub auto_detect_extensions: bool, // true
    pub compatibility_mode: bool,
    pub connection_timeout: Duration,
    pub database: String,
    pub fetch_size: u64,
    pub force_binary: bool,
    pub host: String,
    pub options: HashMap<String, String>,
    pub password: String,
    pub port: u32, // default port
    pub prepared_statement_cache_queries: i32,
    pub schema: String,
    pub socket: String,
    pub ssl_cert: Option<Url>,
    // pub hostname_verifier: HostnameVerifier,
    pub ssl_key: Option<Url>,
    pub ssl_mode: ssl_mode::SslMode, // TODO: expose own so that we can change internals if we need to
    pub ssl_password: String,
    pub ssl_root_cert: Option<Url>,
    pub statement_timeout: Duration,
    pub tcp_keep_alive: bool,
    pub tcp_no_delay: bool, // true
    pub username: String,
}

// example of builder see
// https://github.com/sfackler/rust-native-tls/blob/41522daa6f6e76182c3118a7f9c23f6949e6d59f/src/lib.rs
impl PostgresqlConnectionConfiguration {

    fn new() -> Self {
        Self {
            application_name: "rsdbc-postgresql".to_string(), // TODO: set default somewhere else
            auto_detect_extensions: false,
            compatibility_mode: false,
            connection_timeout: Default::default(),
            database: "".to_string(),
            fetch_size: 0,
            force_binary: false,
            host: "".to_string(),
            options: Default::default(),
            password: "".to_string(),
            port: 5432,
            prepared_statement_cache_queries: 0,
            schema: "".to_string(),
            socket: "".to_string(),
            ssl_cert: None,
            ssl_key: None,
            ssl_mode: ssl_mode::SslMode::Disable,
            ssl_password: "".to_string(),
            ssl_root_cert: None,
            statement_timeout: Default::default(),
            tcp_keep_alive: false,
            tcp_no_delay: false,
            username: "".to_string()
        }
    }

    pub fn application_name(&mut self, name: String) -> &mut Self {
        self.application_name = name;
        self
    }

    // TODO: this might not be necessary....was for java serviceloader
    pub fn auto_detect_extensions(&mut self, auto_detect: bool) -> &mut Self {
        self.auto_detect_extensions = auto_detect;
        self
    }

    pub fn compatibility_mode(&mut self, compatibility_mode: bool) -> &mut Self {
        self.compatibility_mode = compatibility_mode;
        self
    }

    pub fn connect_timeout(&mut self, timeout: Duration) -> &mut Self {
        self.connection_timeout = timeout;
        self
    }

    // TODO: probably don't need
    // pub fn codec_registrar(&mut self, registrar: CodecRegistrar) -> &mut Self {
    //     self.connection_timeout = timeout;
    //     self
    // }

    pub fn database(&mut self, database: String) -> &mut Self {
        self.database = database;
        self
    }

    // TODO: check what ssl modes postgres takes.
    pub fn enable_ssl(&mut self) -> &mut Self {
        self.ssl_mode = ssl_mode::SslMode::Require; // VERIFY_FULL
        self
    }

    // TODO: probably dont need
    // pub fn extend_with(&mut self, extension: Extension) -> &mut Self {
    //     self.extensions.push(extension);
    //     self
    // }

    // TODO: probably don't need
    // pub fn error_response_log_level(&mut self, log_level: LogLevel) -> &mut Self {
    //     self.error_response_log_level = log_level
    //     self
    // }

    pub fn fetch_size(&mut self, fetch_size: u64) -> &mut Self {
        self.fetch_size = fetch_size;
        self
    }

    // TODO: add fn for fetch_size to take in a function?

    pub fn force_binary(&mut self, force_binary: bool) -> &mut Self {
        self.force_binary = force_binary;
        self
    }

    pub fn host(&mut self, host: String) -> &mut Self {
        self.host = host;
        self
    }

    // TODO: probably don't need this
    // pub fn notice_log_level(&mut self, log_level: LogLevel) -> &mut Self {
    //     self.notice_log_level = log_level;
    //     self
    // }

    /// Configure connection initialization parameters.
    ///
    /// These parameters are applied once after creating a new connection.
    /// This is useful for setting up client-specific
    /// <a href="https://www.postgresql.org/docs/current/runtime-config-client.html#RUNTIME-CONFIG-CLIENT-FORMAT">runtime parameters</a>
    /// like statement timeouts, time zones etc.
    pub fn options(&mut self, options: HashMap<String, String>) -> &mut Self {
        self.options = options;
        self
    }

    pub fn password(&mut self, password: String) -> &mut Self {
        self.password = password;
        self
    }

    // TODO: should default to 5432
    pub fn port(&mut self, port: u32) -> &mut Self {
        self.port = port;
        self
    }

    /// Configure the preparedStatementCacheQueries. The default is {@code -1}, meaning there's no limit.
    /// The value of {@code 0} disables the cache. Any other value specifies the cache size.
    pub fn prepared_statement_cache_queries(&mut self, prepared_statement_cache_queries: i32) -> &mut Self {
        self.prepared_statement_cache_queries = prepared_statement_cache_queries;
        self
    }

    pub fn schema(&mut self, schema: String) -> &mut Self {
        self.schema = schema;
        self
    }

    pub fn socket(&mut self, socket: String) -> &mut Self {
        self.socket = socket;
        self.ssl_mode = ssl_mode::SslMode::Disable;
        self
    }

    // TODO: how to handle this failing?
    // Might have to actually use a builder and return Result on call to `build`
    // /// Configure ssl cert for client certificate authentication.
    // /// Can point to either a resource or a file.
    // /// sslCert an X.509 certificate chain file in PEM format
    // pub fn ssl(&mut self, ssl_cert_path: String) -> &mut Self {
    //     self.ssl_url(Url::parse(ssl_cert_path.as_str()))
    // }

    /// Configure ssl cert for client certificate authentication.
    ///
    /// sslCert an X.509 certificate chain file in PEM format
    pub fn ssl_url(&mut self, ssl_cert: Url) -> &mut Self {
        self.ssl_key = Some(ssl_cert);
        self
    }

    // TODO: how to do ssl hostname verifier/verification


    // Configure ssl key for client certificate authentication.
    // Can point to either a resource or a file.
    // pub fn sslkey(&mut self, sslkey: String) -> &mut Self {
    //     self.ssl_key(Url::parse(sslkey.as_str()))
    // }

    /// Configure ssl key for client certificate authentication.
    ///
    /// sslKey a PKCS#8 private key file in PEM format
    pub fn sslkey_url(&mut self, sslkey: Url) -> &mut Self {
        self.ssl_key = Some(sslkey);
        self
    }

    pub fn ssl_mode(&mut self, ssl_mode: ssl_mode::SslMode) -> &mut Self {
        self.ssl_mode = ssl_mode;
        self
    }

    pub fn ssl_password(&mut self, ssl_password: String) -> &mut Self {
        self.ssl_password = ssl_password;
        self
    }

    // Configure ssl root cert for server certificate validation.
    // Can point to either a resource or a file.
    // pub fn ssl_root_cert(&mut self, ssl_root_cert: String) -> &mut Self {
    //     self.ssl_root_cert_url(Url::parse(ssl_root_cert.as_str()))
    // }

    /// Configure ssl root cert for server certificate validation.
    ///
    /// sslRootCert an X.509 certificate chain file in PEM format
    pub fn ssl_root_cert_url(&mut self, ssl_root_cert: Url) -> &mut Self {
        self.ssl_root_cert = Some(ssl_root_cert);
        self
    }

    pub fn tcp_keep_alive(&mut self, enabled: bool) -> &mut Self {
        self.tcp_keep_alive = enabled;
        self
    }

    pub fn tcp_no_delay(&mut self, enabled: bool) -> &mut Self {
        self.tcp_no_delay = enabled;
        self
    }

    pub fn username(&mut self, username: String) -> &mut Self {
        self.username = username;
        self
    }
}

pub struct PostgresqlConnectionFactory {
    pub configuration: PostgresqlConnectionConfiguration
}

pub struct PostgresqlConnection {
    // TODO: does this need to hold ref to configuration?
    client: Client,
    // conn: tokio_postgres::Connection<S, T>,
}

impl Connection for PostgresqlConnection {

    // TODO: provide options to build transaction
    // beginTransaction(TransactionDefinition definition)
    fn begin_transaction(&mut self) -> Result<()> {
        // self.client.transaction()
        // self.client.build_transaction()
        todo!()
    }

    fn close(&mut self) -> Result<()> {
        // self.client.close()
        todo!()
    }

    fn commit_transaction(&mut self) {
        // self.client.
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

    // TODO: not seeing how to do this...needs more research
    fn is_auto_commit(&mut self) -> bool {
        todo!()
    }

    fn metadata(&mut self) -> Result<Box<dyn ConnectionMetadata>> {
        // self.client.
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
        if self.client.is_closed() {
            return false;
        }

        // TODO: where to get duration from?
        // self.client.is_valid(Duration::from_secs(60)).is_ok()

        // "" vs "SELECT 1"

        let query = self.client.simple_query("SELECT 1");
        // tokio::time::timeout(Duration::from_secs(60), query)

        // let inner_client = &self.client;
        // self.connection.block_on(async {
        //     let trivial_query = inner_client.simple_query("");
        //     tokio::time::timeout(timeout, trivial_query)
        //         .await
        //         .map_err(|_| Error::__private_api_timeout())?
        //         .map(|_| ())
        // })


        return true;
    }
}

impl ConnectionFactory for PostgresqlConnectionFactory {
    fn connect(&self) -> Pin<Box<dyn Future<Output = Result<Box<(dyn Connection + 'static)>>> + Send>> {

        // let tls = if self.configuration.ssl_mode == ssl_mode::SslMode::Disable {
        //     NoTls
        // } else {
        //     // let cert = fs::read("database_cert.pem")?;
        //     // let cert = Certificate::from_pem(&cert)?;
        //     // let connector = TlsConnector::builder()
        //     //     .add_root_certificate(cert)
        //     //     .build()?;
        //     // let connector = MakeTlsConnector::new(connector);
        // };

        // let mut client = Client::connect("host=localhost user=postgres", NoTls)?;

        // let (client, connection) =
        //     tokio_postgres::connect("host=localhost user=postgres", NoTls).await.unwrap();

        // The connection object performs the actual communication with the database,
        // so spawn it off to run on its own.
        // tokio::spawn(async move {
        //     if let Err(e) = connection.await {
        //         eprintln!("connection error: {}", e);
        //     }
        // });

        // Client::configure().connect()

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

impl ConnectionFactoryProvider for PostgresqlConnectionFactory {
    type C = PostgresqlConnectionFactory;

    fn create(connection_factory_options: ConnectionFactoryOptions) -> Result<Self::C> {
        let configuration = PostgresqlConnectionConfiguration::new();
        Ok(PostgresqlConnectionFactory {
            configuration
        })
    }
}

fn to_rsdbc_err(e: postgres::error::Error) -> rsdbc_core::error::RsdbcErrors {
    rsdbc_core::error::RsdbcErrors::General(format!("{:?}", e))
}


// pub trait PostgresTransactionDefinition: TransactionDefinition {
//
//     fn deferrable(&mut self) -> &mut Self;
//
//     fn non_deferrable(&mut self) -> &mut Self;
//
//     fn isolation_level(&mut self, isolation_level: IsolationLevel) -> &mut Self {
//         todo!()
//     }
//
//     fn read_only(&mut self) -> &mut Self {
//         todo!()
//     }
//
//     fn read_write(&mut self) -> &mut Self {
//         todo!()
//     }
// }

pub struct PostgresTransactionDefinition {
    pub options: HashMap<String, OptionValue>,
}

impl PostgresTransactionDefinition {

    fn deferrable(&mut self) -> &mut Self {
        self.options.insert("deferrable".to_string(), OptionValue::Bool(true));
        self
    }

    fn non_deferrable(&mut self) -> &mut Self {
        self.options.insert("deferrable".to_string(), OptionValue::Bool(false));
        self
    }

    fn isolation_level(&mut self, isolation_level: IsolationLevel) -> &mut Self {
        self.options.insert("isolationLevel".to_string(), OptionValue::String(isolation_level.as_sql().to_string()));
        self
    }

    fn read_only(&mut self) -> &mut Self {
        self.options.insert("readOnly".to_string(), OptionValue::Bool(true));
        self
    }

    fn read_write(&mut self) -> &mut Self {
        self.options.insert("readOnly".to_string(), OptionValue::Bool(false));
        self
    }

}

impl TransactionDefinition for PostgresTransactionDefinition {
    fn get_attribute(&self, attribute: &str) -> OptionValue {
        todo!()
    }
}

// pub struct SimpleTransactionDefinition {
//     // fn get_attribute<V>(&self, attribute: &str) -> Option<V>;
//     pub options: HashMap<String, String>,
// }
//
// impl TransactionDefinition for SimpleTransactionDefinition {
//     fn get_attribute<V>(&self, attribute: &str) -> Option<V> {
//         todo!()
//     }
// }
//
// impl PostgresTransactionDefinition for SimpleTransactionDefinition {
//     fn deferrable(&mut self) -> &mut Self {
//         todo!()
//     }
//
//     fn non_deferrable(&mut self) -> &mut Self {
//         todo!()
//     }
//
// }


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
