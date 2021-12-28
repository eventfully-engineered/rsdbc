use std::time::Duration;
use url::Url;
use r2dbc_core::connection::{ConnectionFactoryOptions, SslMode};
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

pub struct MySqlConnectionFactory;
impl MySqlConnectionFactory {

    fn create(options: ConnectionFactoryOptions) -> Result<MySqlConnectionFactory> {
        unimplemented!()
    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
