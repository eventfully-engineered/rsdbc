use futures::future::BoxFuture;
use rsdbc_core::connection::{Batch, Connection, ConnectionFactory, ConnectionFactoryMetadata, ConnectionFactoryOptions, ConnectionFactoryProvider, ConnectionMetadata, IsolationLevel, Statement, ValidationDepth};
use rsdbc_core::error::RsdbcErrors;
use rsdbc_core::OptionValue;
use crate::error::SqliteRsdbcError;
use crate::options::SqliteConnectOptions;

pub struct SqliteConnection;
impl Connection for SqliteConnection {
    fn begin_transaction(&mut self) -> rsdbc_core::Result<()> {
        todo!()
    }

    fn close(&mut self) -> rsdbc_core::Result<()> {
        todo!()
    }

    fn commit_transaction(&mut self) {
        todo!()
    }

    fn create_batch(&mut self) -> rsdbc_core::Result<Box<dyn Batch>> {
        todo!()
    }

    fn create_savepoint(&mut self, name: &str) {
        todo!()
    }

    fn create_statement(&mut self, sql: &str) -> rsdbc_core::Result<Box<dyn Statement<'_> + '_>> {
        todo!()
    }

    fn is_auto_commit(&mut self) -> bool {
        todo!()
    }

    fn metadata(&mut self) -> rsdbc_core::Result<Box<dyn ConnectionMetadata>> {
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

pub struct SqliteConnectionMetadata {

}

impl ConnectionMetadata for SqliteConnectionMetadata {
    fn database_product_name(&self) -> &str {
        todo!()
    }

    fn database_version(&self) -> &str {
        todo!()
    }
}



pub struct SqliteConnectionFactory {
    pub configuration: SqliteConnectOptions,
}

impl ConnectionFactory for SqliteConnectionFactory {
    fn connect(&self) -> BoxFuture<'_, rsdbc_core::Result<Box<dyn Connection>>> {
        todo!()
    }

    fn get_metadata(&self) -> Box<dyn ConnectionFactoryMetadata> {
        todo!()
    }
}


// TODO: use From trait instead?
impl ConnectionFactoryProvider for SqliteConnectionFactory {
    type C = SqliteConnectionFactory;

    fn create(connection_factory_options: ConnectionFactoryOptions) -> rsdbc_core::Result<Self::C> {
        // TODO: map options to sqlite options
        // TODO: prefer non-consuming builder - https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
        let mut sqlite_options = SqliteConnectOptions::new();

        // TODO: just testing how this would work
        let protocol = connection_factory_options.options.get("protocol");
        if let Some(protocol) = protocol {
            let protocol_value = match protocol {
                OptionValue::String(s) => {
                    s.to_string()
                }
                _ => {
                    // TODO: return error here
                    "".to_string()
                }
            };
            if protocol_value == "memory" {

            } else {
                sqlite_options = sqlite_options.filename(protocol_value);
            }
        } else {
            return Err(RsdbcErrors::from(SqliteRsdbcError::InvalidProtocol("".to_string())));
        }

        Ok(SqliteConnectionFactory {
            configuration: sqlite_options
        })

    }
}
