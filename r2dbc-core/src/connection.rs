use std::collections::HashMap;
use std::str::FromStr;
use std::time::Duration;
use futures::future::BoxFuture;
use url::Url;
use crate::{OptionValue, R2dbcErrors, Result};

pub trait ConnectionFactory: 'static + Send + Sync {
    // TODO: should have associated type for Error so that we have multiple error types?
    // TODO: remove associated type?
    // type Connection: Connection + ?Sized;

    // // TODO: this was a generic fn/impl
    // // TODO: move out of this trait so that implementations dont worry about this here
    // /// Returns a [ConnectionFactory] from an available implementation, created from a Connection URL.
    // fn new(url: String) -> Result<Box<Self>>;
    //
    // /// Returns a [ConnectionFactory] from an available implementation,
    // /// created from a collection of [ConnectionFactoryOptions].
    // fn from(options: ConnectionFactoryOptions) -> Result<Box<Self>>;

    // TODO: rename to create?
    // /// Establish a new database connection with the options specified by `self`.
    // fn connect(&self) -> BoxFuture<'_, Result<Self::Connection>>
    //     where
    //         Self::Connection: Sized;

    // TODO: create instead of connect?
    /// Establish a new database connection with the options specified by ConnectionOptions.
    fn connect(&self) -> BoxFuture<'_, Result<Box<dyn Connection>>>;

    /// Returns the [ConnectionFactoryMetadata] about the product this [ConnectionFactory] is applicable to.
    fn get_metadata(&self) -> Box<dyn ConnectionFactoryMetadata>;
}

#[derive(Debug, Clone)]
pub struct ConnectionFactoryOptionsBuilder {
    pub options: HashMap<String, OptionValue>,
}

impl ConnectionFactoryOptionsBuilder {

    pub fn new() -> Self {
        Self {
            options: Default::default()
        }
    }

    pub fn build(&self) -> ConnectionFactoryOptions {
        ConnectionFactoryOptions::from(self.options.to_owned())
    }

    pub fn from_options(connection_factory_options: ConnectionFactoryOptions) -> Self {
        let mut options = HashMap::new();
        for (key, value) in connection_factory_options.options {
            options.insert(key, value);
        }

        Self {
            options
        }
    }

    pub fn add_option<K: Into<String>>(&mut self, key: K, value: OptionValue) -> &mut Self {
        self.options.insert(key.into(), value);
        self
    }

    pub fn add_bool<K: Into<String>>(&mut self, key: K, value: bool) -> &mut Self {
        self.options.insert(key.into(), value.into());
        self
    }

    pub fn add_duration<K: Into<String>>(&mut self, key: K, value: Duration) -> &mut Self {
        self.options.insert(key.into(), value.into());
        self
    }

    pub fn i32<K: Into<String>>(&mut self, key: K, value: i32) -> &mut Self {
        self.options.insert(key.into(), value.into());
        self
    }

    pub fn add_map<K: Into<String>>(&mut self, key: K, value: HashMap<String, String>) -> &mut Self {
        self.options.insert(key.into(), value.into());
        self
    }

    pub fn add_string<K: Into<String>>(&mut self, key: K, value: String) -> &mut Self {
        self.options.insert(key.into(), value.into());
        self
    }

}

#[derive(Debug, Clone)]
pub struct ConnectionFactoryOptions {
    pub options: HashMap<String, OptionValue>,
}

// TODO: where should build reside?
impl ConnectionFactoryOptions {

    pub fn new() -> Self {
        Self {
            options: Default::default()
        }
    }

    pub fn from(options: HashMap<String, OptionValue>) -> Self {
        Self {
            options
        }
    }

    pub fn get_value(&self, option: &str) -> Option<&OptionValue> {
        self.options.get(option)
    }

    pub fn try_as_bool(&self, option: &str) -> Result<bool> {
        let value = self.get_value(option);
        if value.is_none() {
            // TODO: missing value?
            return Err(R2dbcErrors::Unsupported("".to_string()));
        }
        match value.unwrap() {
            OptionValue::Bool(v) => {
                Ok(*v)
            }
            OptionValue::Int(v) => {
                Ok(*v != 0)
            }
            OptionValue::String(v) => {
                match v.as_str() {
                    "true" => Ok(true),
                    "false" => Ok(false),
                    "yes" => Ok(true),
                    "no" => Ok(false),
                    "1" => Ok(true),
                    "0" => Ok(false),
                    _ => Err(R2dbcErrors::Unsupported("".to_string()))
                }
            }
            _ => Err(R2dbcErrors::Unsupported("".to_string()))
        }
    }

    pub fn try_as_i32(&self, option: &str) -> Result<i32> {
        let value = self.get_value(option);
        if value.is_none() {
            // TODO: missing value?
            return Err(R2dbcErrors::Unsupported("".to_string()));
        }
        match value.unwrap() {
            OptionValue::Bool(v) => {
                Ok(*v as i32)
            }
            OptionValue::Int(v) => {
                Ok(*v)
            }
            OptionValue::String(v) => {
                // let my_int: i32 = my_string.parse().unwrap();
                Ok(v.parse::<i32>()?)
            }
            _ => Err(R2dbcErrors::Unsupported("".to_string()))
        }
    }


    pub fn has_option(&self, option: &str) -> bool {
        self.options.contains_key(option)
    }

    pub fn parse<S: AsRef<str>>(url: S) -> Result<Self> {
        url.as_ref().parse()
    }
}

impl FromStr for ConnectionFactoryOptions {
    type Err = R2dbcErrors;

    // TODO: clean this up
    fn from_str(s: &str) -> Result<Self> {
        let u = Url::parse(s)?;
        println!("{}", u);
        println!("path: {}", u.path());
        println!("host: {}", u.host_str().unwrap());
        println!("domain: {}", u.domain().unwrap());
        println!("fragment: {}", u.fragment().or(Some("")).unwrap());
        println!("scheme: {}", u.scheme());

        validate(&s)?;

        let scheme_parts: Vec<&str> = s.splitn(3, ":").collect();
        let scheme = scheme_parts[0];
        let driver = scheme_parts[1];
        let protocol = scheme_parts[2];

        // TODO: use .ok_or here instead?
        let scheme_specific_part_index = s.find("://").unwrap();
        let rewritten_url = scheme.to_owned() + &s[scheme_specific_part_index..];

        let uri = Url::parse(rewritten_url.as_str())?;

        // TODO: builder
        let mut connection_factory_builder = ConnectionFactoryOptionsBuilder::new();
        // TODO: ssl?

        connection_factory_builder.add_option("driver", driver.into());

        let protocol_end = protocol.find("://");
        if let Some(protocol_end) = protocol_end {
            let protocol_bits = &protocol[..protocol_end];
            if !protocol_bits.trim().is_empty() {
                connection_factory_builder.add_option("protocol", protocol_bits.into());
            }
        }


        if uri.has_host() {
            connection_factory_builder.add_option("host", uri.host_str().unwrap().into());
            if !uri.username().is_empty() {
                connection_factory_builder.add_option("user", uri.username().into());
            }

            if let Some(password) = uri.password() {
                connection_factory_builder.add_option("password", password.into());
            }
        }

        if let Some(port) = uri.port() {
            connection_factory_builder.add_option("port", port.into());
        }

        // TODO: validate this
        if !uri.path().is_empty() {
            connection_factory_builder.add_option("database", uri.path().into());
        }

        for (k, v) in uri.query_pairs() {
            // TODO: prohibit certain options
            connection_factory_builder.add_option(k, v.into());

        }


        Ok(connection_factory_builder.build())
    }
}

pub trait ConnectionFactoryProvider {
    type C: ConnectionFactory;
    fn create(options: ConnectionFactoryOptions) -> Result<Self::C>;
}


/// Metadata about the product a [ConnectionFactory] is applicable to.
pub trait ConnectionFactoryMetadata {

    /// Returns the name of the product a [ConnectionFactory] can connect to
    fn name(&self) -> String;

}

// TODO: add cancel that returns cancellation token
/// Represents a connection to a database
// pub trait Connection<'conn> {
pub trait Connection: Send {
    // type Statement: Statement<'conn> + ?Sized;

    // trait attributes
    // TransactionDefinition
    // Batch
    // Statement...this could be simple or prepared so probably doesnt work here
    // ConnectionMetadata


    /// Begins a new transaction.
    fn begin_transaction(&mut self) -> Result<()>;

    // TODO: how to handle object safety for this?
    // /// Begins a new transaction.
    // /// Beginning the transaction may fail if the [TransactionDefinition] conflicts with the
    // /// connection configuration.
    // fn begin_transaction_with_definition(&mut self, definition: Box<dyn TransactionDefinition>);


    // Explicitly close this database connection.
    //
    // This method is **not required** for safe and consistent operation. However, it is
    // recommended to call it instead of letting a connection `drop` as the database backend
    // will be faster at cleaning up resources.
    /// Releases this Connection object's database and resources immediately instead of waiting
    /// for them to be automatically released.
    fn close(&mut self) -> Result<()>;

    /// Commits the current transaction.
    fn commit_transaction(&mut self);

    /// Creates a new [Batch] instance for building a batched request.
    fn create_batch(&mut self) -> Result<Box<dyn Batch>>;

    /// Creates a savepoint in the current transaction.
    /// Arguments:
    ///
    /// * `name`: name the name of the savepoint to create.
    ///
    /// UnsupportedOperationException if savepoints are not supported
    fn create_savepoint(&mut self, name: &str);

    /// Creates a new statement for building a statement-based request.
    /// Arguments:
    ///
    /// * `name`: the SQL of the statement
    ///
    // fn create_statement(&mut self, sql: &str) -> Result<Box<Self::Statement>>;
    // rustc --explain E0759
    // to declare that the trait object captures data from argument `self`, you can add an explicit `'_` lifetime bound
    fn create_statement(&mut self, sql: &str) -> Result<Box<dyn Statement<'_> + '_>>;

    /// Returns the auto-commit mode for this connection.
    ///
    /// @return true if the connection is in auto-commit mode; false otherwise.
    fn is_auto_commit(&mut self) -> bool;

    /// Returns the [ConnectionMetadata] about the product this [Connection] is connected to.
    fn metadata(&mut self) -> Result<Box<dyn ConnectionMetadata>>;

    /// Returns the [IsolationLevel] for this connection.
    ///
    /// Isolation level is typically one of the following constants:
    /// - READ_UNCOMMITTED
    /// - READ_COMMITTED
    /// - REPEATABLE_READ
    /// - SERIALIZABLE
    ///
    /// [IsolationLevel] is extensible so drivers can return a vendor-specific [IsolationLevel].
    fn transaction_isolation_level(&mut self) -> IsolationLevel;

    // TODO: This makes sense if the connection is dealing with underlying transaction
    // not sure it makes sense here if we return the transaction to the client
    /// Releases a savepoint in the current transaction.
    /// Calling this for drivers not supporting savepoint release results in a no-op.
    /// Arguments:
    ///
    /// * `name`: the name of the savepoint to release
    fn release_savepoint(&mut self, name: &str);

    /// Rolls back the current transaction.
    fn rollback_transaction(&mut self);

    // TODO: This makes sense if the connection is dealing with underlying transaction
    // not sure it makes sense here if we return the transaction to the client
    /// Rolls back to a savepoint in the current transaction.
    /// Arguments:
    ///
    /// * `name`: the name of the savepoint to rollback to
    ///
    /// @throws UnsupportedOperationException if savepoints are not supported
    fn rollback_transaction_to_savepoint(&mut self, name: String);

    /// Configures the auto-commit mode for the current transaction.
    /// If a connection is in auto-commit mode, then all [Statement]s will be executed
    /// and committed as individual transactions.
    /// Otherwise, in explicit transaction mode, transactions have to
    /// be [beginTransaction()] started explicitly.
    /// A transaction needs to be either [commitTransaction()] committed
    /// or [rollbackTransaction()] rolled back to clean up the transaction state.
    ///
    /// Calling this method during an active transaction and the auto-commit mode is changed,
    /// the transaction is committed.
    /// Calling this method without changing auto-commit mode this invocation results in a no-op.
    ///
    /// Arguments:
    ///
    /// * `name`: the isolation level for this transaction
    fn auto_commit(&mut self, commit: bool);

    /// Configures the isolation level for the current transaction.
    /// Isolation level is typically one of the following constants:
    /// - READ_UNCOMMITTED
    /// - READ_COMMITTED
    /// - REPEATABLE_READ
    /// - SERIALIZABLE
    /// [IsolationLevel] is extensible so drivers can accept a vendor-specific [IsolationLevel].
    /// isolationLevel the isolation level for this transaction
    fn set_transaction_isolation_level(&mut self, isolation_level: IsolationLevel);

    /// Validates the connection according to the given [ValidationDepth].
    /// Emits true if the validation was successful or false if the validation failed.
    /// Does not emit errors and does not complete empty.
    /// Arguments:
    ///
    /// * `depth`: the validation depth
    fn validate(&mut self, depth: ValidationDepth) -> bool;



    // /// Makes all changes made since the previous commit/rollback permanent and releases any database locks currently held by this Connection object.
    // fn commit(&mut self);

// Statement	createStatement()
// Creates a Statement object for sending SQL statements to the database.
// Statement	createStatement(int resultSetType, int resultSetConcurrency)
// Creates a Statement object that will generate ResultSet objects with the given type and concurrency.
// Statement	createStatement(int resultSetType, int resultSetConcurrency, int resultSetHoldability)
// Creates a Statement object that will generate ResultSet objects with the given type, concurrency, and holdability.
//
//     /// Create a statement for execution
//     fn create(&mut self, sql: &str) -> Result<Box<dyn Statement + '_>>;
//
//     /// Retrieves this Connection object's current catalog name.
//     fn get_catalog(&mut self) -> &str;
//
//     /// Returns a list containing the name and current value of each client info property supported by the driver.
//     fn get_all_client_info(&mut self) -> HashMap<String, String>;
//
//     /// Returns the value of the client info property specified by name.
//     fn get_client_info(&mut self, name: &str) -> &str;
//
//     /// Retrieves a DatabaseMetadata object that contains metadata about the database to which this Connection object represents a connection.
//     fn get_metadata(&mut self) -> DatabaseMetadata;
//
//     /// Retrieves the number of milliseconds the driver will wait for a database request to complete.
//     fn get_network_timeout(&mut self) -> i32;
//
//     /// Retrieves this Connection object's current schema name.
//     fn get_schema(&mut self) -> &str;
//
//     /// Retrieves this Connection object's current transaction isolation level.
//     fn get_transaction_isolation(&mut self) -> i32;
//
//     /// Retrieves the first warning reported by calls on this Connection object.
//     fn get_warnings(&mut self) -> SQLWarning;
//
//     /// Retrieves whether this Connection object has been closed.
//     fn is_closed(&mut self) -> bool;
//
//     /// Retrieves whether this Connection object is in read-only mode.
//     fn is_read_only(&mut self) -> bool;
//
//     /// Returns true if the connection has not been closed and is still valid.
//     fn is_valid(&mut self, time_out: i32) -> bool;
//
//     /// Converts the given SQL statement into the system's native SQL grammar.
//     fn native_sql(&mut self, sql: &str) -> &str;
//
//     /// Create a prepared statement for execution
//     fn prepare(&mut self, sql: &str) -> Result<Box<dyn Statement + '_>>;

    // CallableStatement	prepareCall(String sql)
    // Creates a CallableStatement object for calling database stored procedures.
    // CallableStatement	prepareCall(String sql, int resultSetType, int resultSetConcurrency)
    // Creates a CallableStatement object that will generate ResultSet objects with the given type and concurrency.
    // CallableStatement	prepareCall(String sql, int resultSetType, int resultSetConcurrency, int resultSetHoldability)
    // Creates a CallableStatement object that will generate ResultSet objects with the given type and concurrency.
    // PreparedStatement	prepareStatement(String sql)
    // Creates a PreparedStatement object for sending parameterized SQL statements to the database.
    // PreparedStatement	prepareStatement(String sql, int autoGeneratedKeys)
    // Creates a default PreparedStatement object that has the capability to retrieve auto-generated keys.
    // PreparedStatement	prepareStatement(String sql, int[] columnIndexes)
    // Creates a default PreparedStatement object capable of returning the auto-generated keys designated by the given array.
    // PreparedStatement	prepareStatement(String sql, int resultSetType, int resultSetConcurrency)
    // Creates a PreparedStatement object that will generate ResultSet objects with the given type and concurrency.
    // PreparedStatement	prepareStatement(String sql, int resultSetType, int resultSetConcurrency, int resultSetHoldability)
    // Creates a PreparedStatement object that will generate ResultSet objects with the given type, concurrency, and holdability.
    // PreparedStatement	prepareStatement(String sql, String[] columnNames)
    // Creates a default PreparedStatement object capable of returning the auto-generated keys designated by the given array.

    // /// Undoes all changes made in the current transaction and releases any database locks currently held by this Connection object.
    // fn rollback(&mut self);
}

pub trait Transaction {

    // exec
    // prepare
    // query
    // statement

    /// Commits the current transaction.
    fn commit_transaction(&mut self) -> Result<()>;

    /// Creates a new [Batch] instance for building a batched request.
    fn create_batch(&mut self) -> Result<Box<dyn Batch>>;

    /// Creates a savepoint in the current transaction.
    /// Arguments:
    ///
    /// * `name`: name the name of the savepoint to create.
    ///
    /// UnsupportedOperationException if savepoints are not supported
    fn create_savepoint(&mut self, name: &str) -> Result<Box<Self>>;


    /// Releases a savepoint in the current transaction.
    /// Calling this for drivers not supporting savepoint release results in a no-op.
    /// Arguments:
    ///
    /// * `name`: the name of the savepoint to release
    fn release_savepoint(&mut self, name: &str);

    /// Rolls back the current transaction.
    fn rollback_transaction(&mut self) -> Result<()>;


}


fn validate(url: &str) -> Result<()> {
    Ok(())
}

/// Metadata about the product a [Connection] is connected to.
pub trait ConnectionMetadata {

    /// Retrieves the name of this database product.
    /// May contain additional information about editions.
    fn database_product_name(&self) -> &str;

    /// Retrieves the version number of this database product.
    fn database_version(&self) -> &str;
}

/// A collection of statements that are executed in a batch for performance reasons.
pub trait Batch {

    /// Add a statement to this batch.
    fn add(&mut self, sql: String) -> &mut Self where Self: Sized;

    /// Executes one or more SQL statements and returns the [Result]s.
    fn execute(&mut self) -> Result<Box<dyn SQLResult>>;
}

// TODO: Should this include None or just use Option? I'm currently leaning Option
/// Represents a transaction isolation level constant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum IsolationLevel {
    /// The read committed isolation level.
    ReadCommitted,
    /// The read uncommitted isolation level.
    ReadUncommitted,
    /// The repeatable read isolation level.
    RepeatableRead,
    /// The serializable isolation level.
    Serializable
}

impl IsolationLevel {
    pub(crate) fn new(raw: &str) -> Result<IsolationLevel> {
        if raw.eq_ignore_ascii_case("READ UNCOMMITTED") {
            Ok(IsolationLevel::ReadUncommitted)
        } else if raw.eq_ignore_ascii_case("READ COMMITTED") {
            Ok(IsolationLevel::ReadCommitted)
        } else if raw.eq_ignore_ascii_case("REPEATABLE READ") {
            Ok(IsolationLevel::RepeatableRead)
        } else if raw.eq_ignore_ascii_case("SERIALIZABLE") {
            Ok(IsolationLevel::Serializable)
        } else {
            // Err(bad_response().into())
            // Err(Error::Io(bad_response()))
            // Err(io::Error::new(
            //     io::ErrorKind::InvalidInput,
            //     "the server returned an unexpected response",
            // ))
            Err(R2dbcErrors::General(String::from("the server returned an unexpected response")))
        }
    }

    // TODO: review https://rust-lang.github.io/api-guidelines/naming.html#ad-hoc-conversions-follow-as_-to_-into_-conventions-c-conv
    pub fn as_sql(&self) -> &'static str {
        match *self {
            IsolationLevel::ReadUncommitted => "READ UNCOMMITTED",
            IsolationLevel::ReadCommitted => "READ COMMITTED",
            IsolationLevel::RepeatableRead => "REPEATABLE READ",
            IsolationLevel::Serializable => "SERIALIZABLE",
        }
    }
}


pub trait SQLResult {
    fn get_rows_updated(&self) -> Option<u32>;

    // TODO: map function

    // <T> Publisher<T> map(BiFunction<Row, RowMetadata, ? extends T> mappingFunction);
    // fn map<F, B>(self, f: F) -> MappedRows<'stmt, F>
    //     where
    //         F: FnMut(&dyn Row<'_>) -> Result<B>,
    // {
    //     MappedRows { rows: self, map: f }
    // }
}

/// Constants indicating validation depth for a [Connection].
pub enum ValidationDepth {
    /// Perform a client-side only validation.
    /// Typically to determine whether a connection is still active or other mechanism
    /// that does not involve remote communication.
    Local,
    /// Perform a remote connection validations.
    /// Typically by sending a database message or some other mechanism to validate that
    /// the database connection and session are active and can be used for
    /// database queries.
    /// Any query submitted by the driver to validate the connection is executed in
    /// the context of the current transaction.
    Remote,
}


/// Represents an executable statement
pub trait Statement<'conn> {

    // from java r2dbc
    fn add(&mut self) -> &mut Self where Self: Sized; //Box<dyn A>

    fn bind_index<T>(&mut self, index: u32, value: T) -> &mut Self where Self: Sized; //Box<dyn A>

    fn bind_name<T>(&mut self, name: &str, value: T) -> &mut Self where Self: Sized; //Box<dyn A>

    // TODO: not sure what type should be here
    // these might not be needed
    // removed type for now
    fn bind_null_index(&mut self, index: u32) -> &mut Self where Self: Sized; //Box<dyn A>
    fn bind_null_name(&mut self, name: &str) -> &mut Self where Self: Sized; //Box<dyn A>

    // TODO: should be a stream?
    // not sure about this where Self: Sized
    fn execute<T: SQLResult>(&self) -> Result<T> where Self: Sized;

    /// Configures [Statement] to return the generated values from any rows created by this
    /// [Statement] in the [SQLResult] returned from [execute()].
    /// If no columns are specified, implementations are free to choose which columns
    /// will be returned.
    /// If called multiple times, only the columns requested in the final invocation will be returned.
    ///
    /// The default implementation of this method is a no-op.
    fn return_generated_values(&mut self, columns: &[&str]) -> &mut Self where Self: Sized { //Box<dyn A>
        // default is no-op
        self
    }

    /// Configures [Statement] to retrieve a fixed number of rows when fetching results from a
    /// query instead deriving fetch size from back pressure.
    /// If called multiple times, only the fetch size configured in the final invocation
    /// will be applied.
    /// If the value specified is zero, then the hint is ignored.
    /// The default implementation of this method is a no op and the default value is zero.
    fn fetch_size(&mut self, rows: u32) -> &mut Self where Self: Sized { //Box<dyn A>
        // The default implementation of this method is a no op and the default value is zero.
        self
    }



    // /// Execute a query that is expected to return a result set, such as a `SELECT` statement
    // fn execute_query(&mut self, params: &[Value]) -> Result<Box<dyn ResultSet + '_>>;
    //
    // /// Execute a query that is expected to update some rows.
    // fn execute_update(&mut self, params: &[Value]) -> Result<u64>;
}


// TODO: each db probably has a different set so this probably doesnt make sense as an enum here
#[derive(Debug, Copy, Clone, PartialEq)]
#[non_exhaustive]
pub enum SslMode {
    /// Do not use TLS.
    Disable,
    /// Attempt to connect with TLS but allow sessions without.
    Prefer,
    /// Require the use of TLS.
    Require,
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::connection::{ConnectionFactoryOptions, ConnectionFactoryOptionsBuilder};
    use crate::R2dbcErrors;

    #[test]
    fn programmatic_connection_factory_builder() {
        let options = HashMap::from([
            ("lock_timeout", "10s"),
            ("statement_timeout", "5m"),
        ]);

        let connection_factory_options = ConnectionFactoryOptionsBuilder::new()
            .add_option("driver", "postgresql".into())
            .add_string("localhost", "localhost".to_string())
            .add_option("port", 5432.into())
            .add_option("options", options.into())
            .build();

        assert_eq!(5432, connection_factory_options.try_as_i32("port").unwrap());
    }

    #[test]
    fn missing_connection_factory_option_should_return_none() {
        let connection_factory_options = ConnectionFactoryOptionsBuilder::new().build();

        assert_eq!(None, connection_factory_options.get_value("driver"));
    }

    #[test]
    fn connection_factory_has_option_should_return_appropriate_bool() {
        let connection_factory_options = ConnectionFactoryOptionsBuilder::new()
            .add_option("driver", "postgresql".into())
            .build();

        assert!(connection_factory_options.has_option("driver"));
        assert!(!connection_factory_options.has_option("port"));
    }

}
