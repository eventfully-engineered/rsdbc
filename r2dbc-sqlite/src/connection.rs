use futures::future::BoxFuture;
use r2dbc_core::connection::{Batch, Connection, ConnectionFactory, ConnectionFactoryMetadata, ConnectionFactoryOptions, ConnectionFactoryProvider, ConnectionMetadata, IsolationLevel, Statement, ValidationDepth};
use r2dbc_core::error::R2dbcErrors;
use crate::error::SqliteR2dbcError;
use crate::options::SqliteConnectOptions;

pub struct SqliteConnection;
impl Connection for SqliteConnection {
    fn begin_transaction(&mut self) -> r2dbc_core::Result<()> {
        todo!()
    }

    fn close(&mut self) -> r2dbc_core::Result<()> {
        todo!()
    }

    fn commit_transaction(&mut self) {
        todo!()
    }

    fn create_batch(&mut self) -> r2dbc_core::Result<Box<dyn Batch>> {
        todo!()
    }

    fn create_savepoint(&mut self, name: &str) {
        todo!()
    }

    fn create_statement(&mut self, sql: &str) -> r2dbc_core::Result<Box<dyn Statement<'_> + '_>> {
        todo!()
    }

    fn is_auto_commit(&mut self) -> bool {
        todo!()
    }

    fn metadata(&mut self) -> r2dbc_core::Result<Box<dyn ConnectionMetadata>> {
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
    fn connect(&self) -> BoxFuture<'_, r2dbc_core::Result<Box<dyn Connection>>> {
        todo!()
    }

    fn get_metadata(&self) -> Box<dyn ConnectionFactoryMetadata> {
        todo!()
    }
}


// TODO: use From trait instead?
impl ConnectionFactoryProvider for SqliteConnectionFactory {
    type C = SqliteConnectionFactory;

    fn create(connection_factory_options: ConnectionFactoryOptions) -> r2dbc_core::Result<Self::C> {
        // TODO: map options to sqlite options
        // TODO: prefer non-consuming builder - https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
        let mut sqlite_options = SqliteConnectOptions::new();

        // TODO: just testing how this would work
        let protocol = connection_factory_options.options.get("protocol");
        if let Some(protocol) = protocol {
            if protocol == "memory" {

            } else {
                sqlite_options = sqlite_options.filename(protocol);
            }
        } else {
            return Err(R2dbcErrors::from(SqliteR2dbcError::InvalidProtocol("".to_string())));
        }

        Ok(SqliteConnectionFactory {
            configuration: sqlite_options
        })

    }
}
