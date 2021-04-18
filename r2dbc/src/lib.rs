use std::collections::HashMap;
use std::error::Error as StdError;
use std::io;

#[derive(Debug, Clone)]
pub enum Value {
    Int32(i32),
    UInt32(u32),
    String(String),
    //TODO add other types
}

/// R2DBC Error
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    Configuration(String),
    General(String),
    Unsupported(String),
}

impl Error {

    // #[allow(dead_code)]
    // #[inline]
    // pub(crate) fn config(err: impl StdError + Send + Sync + 'static) -> Self {
    //     Error::Configuration(err.into())
    // }

    #[allow(dead_code)]
    #[inline]
    pub fn config(err: String) -> Self {
        Error::Configuration(err)
    }
}

pub enum R2dbcError {
    BadGrammar,
    General,
    NonTransient,
    NonTransientResource,
    PermissionDenied,
    Rollback,
    Timeout,
    Transient,
    TransientResource
}


// TODO: maybe enum?
pub struct SQLWarning {

}


/// R2DBC Result type
pub type Result<T> = std::result::Result<T, Error>;

/// Represents database driver that can be shared between threads, and can therefore implement
/// a connection pool
pub trait Driver: Sync + Send {
    /// Create a connection to the database. Note that connections are intended to be used
    /// in a single thread since most database connections are not thread-safe
    fn connect(&self, url: &str, properties: HashMap<String, String>) -> Result<Box<dyn Connection>>;

    /// Retrieves whether the driver thinks that it can open a connection to the given URL.
    fn accepts_url(&self, url: &str) -> bool;

    /// Retrieves the driver's major version number.
    fn get_major_version(&self) -> i32;

    /// Gets the driver's minor version number.
    fn get_minor_version(&self) -> i32;

    // Gets information about the possible properties for this driver.
    // fn get_property_info(&self, url: &str, info: HashMap) -> DriverPropertyInfo
}

pub struct ConfigurationOption {

}

/// Specification of properties to be used when starting a transaction.
/// This interface is typically implemented by code that calls [beginTransaction(TransactionDefinition)]
pub trait TransactionDefinition {
    // /**
    //  * Isolation level requested for the transaction.
    //  */
    // Option<IsolationLevel> ISOLATION_LEVEL = Option.valueOf("isolationLevel");
    //
    // /**
    //  * The transaction mutability (i.e. whether the transaction should be started in read-only mode).
    //  */
    // Option<Boolean> READ_ONLY = Option.valueOf("readOnly");
    //
    // /**
    //  * Name of the transaction.
    //  */
    // Option<String> NAME = Option.valueOf("name");
    //
    // /**
    //  * Lock wait timeout.
    //  */
    // Option<Duration> LOCK_WAIT_TIMEOUT = Option.valueOf("lockWaitTimeout");
    //
    // /**
    //  * Retrieve a transaction attribute by its {@link Option} identifier.  This low-level interface allows querying transaction attributes supported by the {@link Connection} that should be applied
    //  * when starting a new transaction.
    //  *
    //  * @param option the option to retrieve the value for
    //  * @param <T>    requested value type
    //  * @return the value of the transaction attribute. Can be {@code null} to indicate absence of the attribute.
    //  * @throws IllegalArgumentException if {@code name} or {@code type} is {@code null}
    //  */
    // @Nullable
    // <T> T getAttribute(Option<T> option);
}

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
            Err(Error::General(String::from("the server returned an unexpected response")))
        }
    }

    fn to_sql(&self) -> &'static str {
        match *self {
            IsolationLevel::ReadUncommitted => "READ UNCOMMITTED",
            IsolationLevel::ReadCommitted => "READ COMMITTED",
            IsolationLevel::RepeatableRead => "REPEATABLE READ",
            IsolationLevel::Serializable => "SERIALIZABLE",
        }
    }
}

impl TransactionDefinition for IsolationLevel {

}

/// A collection of statements that are executed in a batch for performance reasons.
pub trait Batch {

    /// Add a statement to this batch.
    fn add(&mut self, sql: String) -> &mut Self where Self: Sized;

    /// Executes one or more SQL statements and returns the [Result]s.
    fn execute(&mut self) -> Result<Box<dyn SQLResult>>;
}

/// Metadata about the product a [Connection] is connected to.
pub trait ConnectionMetadata {

    /// Retrieves the name of this database product.
    /// May contain additional information about editions.
    fn database_product_name(&self) -> &str;

    /// Retrieves the version number of this database product.
    fn database_version(&self) -> &str;
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

/// Represents a connection to a database
pub trait Connection {

    /// Begins a new transaction.
    fn begin_transaction(&mut self) -> Result<()>;

    /// Begins a new transaction.
    /// Beginning the transaction may fail if the [TransactionDefinition] conflicts with the
    /// connection configuration.
    fn begin_transaction_with_definition(&mut self, definition: &dyn TransactionDefinition);

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
    fn create_savepoint(&mut self, name: String);

    /// Creates a new statement for building a statement-based request.
    /// Arguments:
    ///
    /// * `name`: the SQL of the statement
    ///
    fn create_statement(&mut self, sql: String) -> Result<Box<dyn Statement>>;

    /// Returns the auto-commit mode for this connection.
    ///
    /// @return true if the connection is in auto-commit mode; false otherwise.
    fn is_auto_commit(&self) -> bool;

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

    /// Releases a savepoint in the current transaction.
    /// Calling this for drivers not supporting savepoint release results in a no-op.
    /// Arguments:
    ///
    /// * `name`: the name of the savepoint to release
    fn release_savepoint(&mut self, name: String);

    /// Rolls back the current transaction.
    fn rollback_transaction(&mut self);

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
    fn validate(&mut self, depth: ValidationDepth);



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

/// Represents an executable statement
pub trait Statement {

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

/// An iterator over the mapped resulting rows of a query.
///
/// `F` is used to transform the _streaming_ iterator into a _standard_ iterator.
// #[must_use = "iterators are lazy and do nothing unless consumed"]
// pub struct MappedRows<'stmt, F> {
//     rows: Rows<'stmt>,
//     map: F,
// }
//
// impl<T, F> Iterator for MappedRows<'_, F>
//     where
//         F: FnMut(&dyn Row<'_>) -> Result<T>,
// {
//     type Item = Result<T>;
//
//     #[inline]
//     fn next(&mut self) -> Option<Result<T>> {
//         let map = &mut self.map;
//         self.rows
//             .next()
//             .transpose()
//             .map(|row_result| row_result.and_then(|row| (map)(&row)))
//     }
// }

/// Result set from executing a query against a statement
pub trait ResultSet {
    /// get meta data about this result set
    fn meta_data(&self) -> Result<Box<dyn ResultSetMetaData>>;

    /// Move the cursor to the next available row if one exists and return true if it does
    fn next(&mut self) -> bool;

    fn get_bool(&self, i: u64) -> Result<Option<bool>>;
    fn get_i8(&self, i: u64) -> Result<Option<i8>>;
    fn get_i16(&self, i: u64) -> Result<Option<i16>>;
    fn get_i32(&self, i: u64) -> Result<Option<i32>>;
    fn get_i64(&self, i: u64) -> Result<Option<i64>>;
    fn get_f32(&self, i: u64) -> Result<Option<f32>>;
    fn get_f64(&self, i: u64) -> Result<Option<f64>>;
    fn get_string(&self, i: u64) -> Result<Option<String>>;
    fn get_bytes(&self, i: u64) -> Result<Option<Vec<u8>>>;
}



/// Meta data for result set
pub trait ResultSetMetaData {
    fn num_columns(&self) -> u64;
    fn column_name(&self, i: u64) -> String;
    fn column_type(&self, i: u64) -> DataType;
    fn column_type_name(&self, i: u64) -> String;
    fn precision(&self, i: u64) -> u64;
    fn schema_name(&self, i: u64) -> String;
    fn table_name(&self, i: u64) -> String;
    fn is_nullable(&self, i: u64) -> String;
    fn is_read_only(&self, i: u64) -> String;
}

pub trait Row<'stmt> {
    fn get_via_index<R>(&self, index: u32) -> R;

    fn get_via_name<R>(&self, name: &str) -> R;
}

// TODO: do we want this generic trait or do we want something more specific like ColumnMetadata
// Java R2DBC has this has database / sql types called Type
// Java R2DBC has an enum that implements interface
// R2dbcType is Definition of generic SQL types
// Type descriptor for column- and parameter types.
// SQLX - Provides information about a SQL type for the database driver.
pub trait TypeInfo {

    // might not need this
    // https://stackoverflow.com/questions/21747136/how-do-i-print-the-type-of-a-variable-in-rust
    fn rust_type(&self) -> &'static str;

    fn name(&self) -> &str;
}

pub enum R2dbcType {
    Char,
    Varchar,
    Nchar,
    Nvarchar,
    Clob,
    Nclob,
    Boolean,
    Varbinary,
    Blob,
    Integer,
    Tinyint,
    Smallint,
    Bigint,
    Numeric,
    Decimal,
    Float,
    Real,
    Double,
    Date,
    Time,
    TimeWithTimeZone,
    Timestamp,
    TimestampWithTimeZone,
    Collection,
}

impl TypeInfo for R2dbcType {
    fn rust_type(&self) -> &'static str {
        todo!()
    }

    fn name(&self) -> &str {
        match self {
            R2dbcType::Char => "CHAR",
            R2dbcType::Varchar => "VARCHAR",
            R2dbcType::Nchar => "NCHAR",
            R2dbcType::Nvarchar => "NVARCHAR",
            R2dbcType::Clob => "CLOB",
            R2dbcType::Nclob => "NCLOB",
            R2dbcType::Boolean => "BOOLEAN",
            R2dbcType::Varbinary => "VARBINARY",
            R2dbcType::Blob => "BLOB",
            R2dbcType::Integer => "INTEGER",
            R2dbcType::Tinyint => "TINYINT",
            R2dbcType::Smallint => "SMALLINT",
            R2dbcType::Bigint => "BIGINT",
            R2dbcType::Numeric => "NUMERIC",
            R2dbcType::Decimal => "DECIMAL",
            R2dbcType::Float => "FLOAT",
            R2dbcType::Real => "REAL",
            R2dbcType::Double => "DOUBLE",
            R2dbcType::Date => "DATE",
            R2dbcType::Time => "TIME",
            R2dbcType::TimeWithTimeZone => "TIME_WITH_TIME_ZONE",
            R2dbcType::Timestamp => "TIMESTAMP",
            R2dbcType::TimestampWithTimeZone => "TIMESTAMP_WITH_TIME_ZONE",
            R2dbcType::Collection => "COLLECTION",
        }
    }
}


/// Represents the metadata for a column of the results returned from a query.
/// The implementation of all methods except [getName()] is optional for drivers.
/// Column metadata is optionally available as by-product of statement execution on a best-effort basis.
pub trait ColumnMetadata {

    /// returns the database Type [TypeInfo]
    fn db_type(&self) -> dyn TypeInfo;

    fn name(&self) -> &str;


    // Returns the native type descriptor that potentially exposes more metadata.
    // Drivers should implement this method if they can expose a driver-specific type metadata object exposing additional information.  The default implementation returns {@code null}.
    //
    // @return the native type descriptor that potentially exposes more metadata or {@code null} if no native type descriptor is available.
    //
    // native_type_metadata()

    fn nullability(&self) -> Nullability {
        Nullability::Unknown
    }

    fn precision(&self) -> Option<u64> {
        None
    }

    fn scale(&self) -> Option<u64> {
        None
    }
}

pub enum Nullability {
    Nullable,
    NonNull,
    Unknown
}


/// RDBC Data Types
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DataType {
    Bool,
    Byte,
    Char,
    Short,
    Integer,
    Float,
    Double,
    Decimal,
    Date,
    Time,
    Datetime,
    Utf8,
    Binary,
}

#[derive(Debug, Clone)]
pub struct Column {
    name: String,
    data_type: DataType,
    //precision: u6,
}

impl Column {
    pub fn new(name: &str, data_type: DataType) -> Self {
        Column {
            name: name.to_owned(),
            data_type,
        }
    }
}

impl ResultSetMetaData for Vec<Column> {
    fn num_columns(&self) -> u64 {
        self.len() as u64
    }

    fn column_name(&self, i: u64) -> String {
        self[i as usize].name.clone()
    }

    fn column_type(&self, i: u64) -> DataType {
        self[i as usize].data_type
    }

    // fn precision(&self, i: u64) -> u64 {
    //     match self.column_type {

    //     }
    // }

    fn column_type_name(&self, i: u64) -> String {
        todo!()
    }

    fn precision(&self, i: u64) -> u64 {
        todo!()
    }

    fn schema_name(&self, i: u64) -> String {
        todo!()
    }

    fn table_name(&self, i: u64) -> String {
        todo!()
    }

    fn is_nullable(&self, i: u64) -> String {
        todo!()
    }

    fn is_read_only(&self, i: u64) -> String {
        todo!()
    }
}

pub trait DatabaseMetadata {

}