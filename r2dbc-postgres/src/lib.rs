use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::time::Duration;
use futures::future::BoxFuture;
use postgres::config::SslMode;
use tracing_subscriber::fmt::time;
use url::Url;
use r2dbc_core::connection::{Batch, Connection, ConnectionFactory, ConnectionFactoryMetadata, ConnectionFactoryOptions, ConnectionFactoryProvider, ConnectionMetadata, IsolationLevel, Statement, ValidationDepth};
use r2dbc_core::error::R2dbcErrors;
use r2dbc_core::Result;

pub struct  PostgresqlConnectionConfiguration {
    // application_name = "r2dbc-postgresql"
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
    pub ssl_mode: SslMode, // TODO: expose own so that we can change internals if we need to
    pub ssl_password: String,
    pub ssl_root_cert: Option<Url>,
    pub statement_timeout: Duration,
    pub tcp_keep_alive: bool,
    pub tcp_no_delay: bool, // true
    pub username: String,
}

impl PostgresqlConnectionConfiguration {

    fn new() -> Self {
        Self {
            application_name: "r2dbc-postgresql".to_string(), // TODO: set default somewhere else
            auto_detect_extensions: false,
            compatibility_mode: false,
            connection_timeout: Default::default(),
            database: "".to_string(),
            fetch_size: 0,
            force_binary: false,
            host: "".to_string(),
            options: Default::default(),
            password: "".to_string(),
            port: 0,
            prepared_statement_cache_queries: 0,
            schema: "".to_string(),
            socket: "".to_string(),
            ssl_cert: None,
            ssl_key: None,
            ssl_mode: SslMode::Disable,
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
        self.ssl_mode = SslMode::Require; // VERIFY_FULL
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

    // TODO: WTF is this?
    // pub fn loop_resources(&mut self, loop_resources: LoopResources) -> &mut Self {
    //     self.loop_resources = loop_resources;
    //     self
    // }

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

    // pub fn prefer_attached_buffers(&mut self, prefer_attached_buffers: bool) -> &mut Self {
    //     self.prefer_attached_buffers = prefer_attached_buffers;
    //     self
    // }

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
        self.ssl_mode = SslMode::Disable;
        self
    }


    // /**
    //  * Configure a {@link SslContextBuilder} customizer. The customizer gets applied on each SSL connection attempt to allow for just-in-time configuration updates. The {@link Function} gets
    //  * called with the prepared {@link SslContextBuilder} that has all configuration options applied. The customizer may return the same builder or return a new builder instance to be used to
    //  * build the SSL context.
    //  *
    //  * @param sslContextBuilderCustomizer customizer function
    //  * @return this {@link Builder}
    //  * @throws IllegalArgumentException if {@code sslContextBuilderCustomizer} is {@code null}
    //  */
    // public Builder sslContextBuilderCustomizer(Function<SslContextBuilder, SslContextBuilder> sslContextBuilderCustomizer) {
    // this.sslContextBuilderCustomizer = Assert.requireNonNull(sslContextBuilderCustomizer, "sslContextBuilderCustomizer must not be null");
    // return this;
    // }
    //
    // /**
    //  * Configure ssl cert for client certificate authentication. Can point to either a resource within the classpath or a file.
    //  *
    //  * @param sslCert an X.509 certificate chain file in PEM format
    //  * @return this {@link Builder}
    //  */
    // public Builder sslCert(String sslCert) {
    // return sslCert(requireExistingFilePath(sslCert, "sslCert must not be null and must exist"));
    // }
    //
    // /**
    //  * Configure ssl cert for client certificate authentication.
    //  *
    //  * @param sslCert an X.509 certificate chain file in PEM format
    //  * @return this {@link Builder}
    //  * @since 0.8.7
    //  */
    // public Builder sslCert(URL sslCert) {
    // this.sslCert = Assert.requireNonNull(sslCert, "sslCert must not be null");
    // return this;
    // }
    //
    // /**
    //  * Configure ssl HostnameVerifier.
    //  *
    //  * @param sslHostnameVerifier {@link HostnameVerifier}
    //  * @return this {@link Builder}
    //  */
    // public Builder sslHostnameVerifier(HostnameVerifier sslHostnameVerifier) {
    // this.sslHostnameVerifier = Assert.requireNonNull(sslHostnameVerifier, "sslHostnameVerifier must not be null");
    // return this;
    // }
    //
    // /**
    //  * Configure ssl key for client certificate authentication.  Can point to either a resource within the classpath or a file.
    //  *
    //  * @param sslKey a PKCS#8 private key file in PEM format
    //  * @return this {@link Builder}
    //  */
    // public Builder sslKey(String sslKey) {
    // return sslKey(requireExistingFilePath(sslKey, "sslKey must not be null and must exist"));
    // }
    //
    // /**
    //  * Configure ssl key for client certificate authentication.
    //  *
    //  * @param sslKey a PKCS#8 private key file in PEM format
    //  * @return this {@link Builder}
    //  * @since 0.8.7
    //  */
    // public Builder sslKey(URL sslKey) {
    // this.sslKey = Assert.requireNonNull(sslKey, "sslKey must not be null");
    // return this;
    // }
    //
    // /**
    //  * Configure ssl mode.
    //  *
    //  * @param sslMode the SSL mode to use.
    //  * @return this {@link Builder}
    //  */
    // public Builder sslMode(SSLMode sslMode) {
    // this.sslMode = Assert.requireNonNull(sslMode, "sslMode must be not be null");
    // return this;
    // }
    //
    // /**
    //  * Configure ssl password.
    //  *
    //  * @param sslPassword the password of the sslKey, or null if it's not password-protected
    //  * @return this {@link Builder}
    //  */
    // public Builder sslPassword(@Nullable CharSequence sslPassword) {
    // this.sslPassword = sslPassword;
    // return this;
    // }
    //
    // /**
    //  * Configure ssl root cert for server certificate validation. Can point to either a resource within the classpath or a file.
    //  *
    //  * @param sslRootCert an X.509 certificate chain file in PEM format
    //  * @return this {@link Builder}
    //  */
    // public Builder sslRootCert(String sslRootCert) {
    // return sslRootCert(requireExistingFilePath(sslRootCert, "sslRootCert must not be null and must exist"));
    // }
    //
    // /**
    //  * Configure ssl root cert for server certificate validation.
    //  *
    //  * @param sslRootCert an X.509 certificate chain file in PEM format
    //  * @return this {@link Builder}
    //  * @since 0.8.7
    //  */
    // public Builder sslRootCert(URL sslRootCert) {
    // this.sslRootCert = Assert.requireNonNull(sslRootCert, "sslRootCert must not be null and must exist");
    // return this;
    // }
    //
    // /**
    //  * Configure TCP KeepAlive.
    //  *
    //  * @param enabled whether to enable TCP KeepAlive
    //  * @return this {@link Builder}
    //  * @see Socket#setKeepAlive(boolean)
    //  * @since 0.8.4
    //  */
    // public Builder tcpKeepAlive(boolean enabled) {
    // this.tcpKeepAlive = enabled;
    // return this;
    // }
    //
    // /**
    //  * Configure TCP NoDelay.
    //  *
    //  * @param enabled whether to enable TCP NoDelay
    //  * @return this {@link Builder}
    //  * @see Socket#setTcpNoDelay(boolean)
    //  * @since 0.8.4
    //  */
    // public Builder tcpNoDelay(boolean enabled) {
    // this.tcpNoDelay = enabled;
    // return this;
    // }
    //
    // /**
    //  * Configure the username.
    //  *
    //  * @param username the username
    //  * @return this {@link Builder}
    //  * @throws IllegalArgumentException if {@code username} is {@code null}
    //  */
    // public Builder username(String username) {
    // this.username = Assert.requireNonNull(username, "username must not be null");
    // return this;
    // }


}

pub struct PostgresqlConnectionFactory {
    pub configuration: PostgresqlConnectionConfiguration
}

pub struct PostgresqlConnection;

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

impl ConnectionFactoryProvider for PostgresqlConnectionFactory {
    type C = PostgresqlConnectionFactory;

    fn create(connection_factory_options: ConnectionFactoryOptions) -> Result<Self::C> {
        let configuration = PostgresqlConnectionConfiguration::new();
        Ok(PostgresqlConnectionFactory {
            configuration
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
