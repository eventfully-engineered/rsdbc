use std::borrow::Cow;
use std::collections::HashMap;
use std::time::Duration;
use url::Url;
use crate::error::R2dbcErrors;

pub mod error;
pub mod connection;

/// R2DBC Result type
pub type Result<T> = std::result::Result<T, R2dbcErrors>;


#[derive(Debug, Clone)]
pub enum Value {
    Int32(i32),
    UInt32(u32),
    String(String),
    // TODO: add other types
}




// TODO: maybe enum?
pub struct SQLWarning {

}


pub struct ConfigurationOption {

}

// Golang has TxOptions
/// Specification of properties to be used when starting a transaction.
/// This interface is typically implemented by code that calls [beginTransaction(TransactionDefinition)]
pub trait TransactionDefinition {

    // TODO: This might have to return Option<&V> like HashMap
    /// Retrieve a transaction attribute value by its attribute identifier.
    /// This low-level interface allows querying transaction attributes supported by the {@link Connection} that should be applied
    ///
    /// returns the value of the transaction attribute. Can be None to indicate absence of the attribute
    fn get_attribute(&self, attribute: &str) -> OptionValue;
}

// TODO: where to put constants?
pub struct TransactionOptions;
impl TransactionOptions {
    /// Isolation level requested for the transaction.
    const ISOLATION_LEVEL: &'static str = "isolation_level";

    /// The transaction mutability (i.e. whether the transaction should be started in read-only mode)
    const READ_ONLY: &'static str = "read_only";

    /// Name of the transaction.
    const NAME: &'static str = "name";

    ///
    const LOCK_WAIT_TIMEOUT: &'static str = "lock_wait_timeout";
}



// TODO: Rename to ConfigurationValue??
// TODO: improve experience with &str
// TODO: add convenience methods to get values of a certain type back. Should be Result<blah>. TryInto?
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum OptionValue {
    Int(i32),
    Bool(bool),
    String(String),
    Duration(Duration), // Chrono Duration? Should that be a feature?
    Map(HashMap<String, String>),
}

// TODO: this might be a good time to use a macro
// TODO: implement others
impl From<i32> for OptionValue {
    fn from(value: i32) -> Self {
        OptionValue::Int(value)
    }
}

impl From<u16> for OptionValue {
    fn from(value: u16) -> Self {
        OptionValue::Int(value as i32)
    }
}

impl From<bool> for OptionValue {
    fn from(value: bool) -> Self {
        OptionValue::Bool(value)
    }
}

impl From<Cow<'_, str>> for OptionValue {
    fn from(value: Cow<'_, str>) -> Self {
        OptionValue::String(value.to_string())
    }
}

impl From<String> for OptionValue {
    fn from(value: String) -> Self {
        OptionValue::String(value)
    }
}

impl From<&str> for OptionValue {
    fn from(value: &str) -> Self {
        OptionValue::String(value.to_string())
    }
}

impl From<Duration> for OptionValue {
    fn from(value: Duration) -> Self {
        OptionValue::Duration(value)
    }
}

impl From<Url> for OptionValue {
    fn from(value: Url) -> Self {
        OptionValue::String(value.to_string())
    }
}

impl From<HashMap<String, String>> for OptionValue {
    fn from(value: HashMap<String, String>) -> Self {
        OptionValue::Map(value)
    }
}

fn parse_connection(url: &str) {
    // TODO: parse url to Connection options
}

// TODO: explore this
// https://github.com/launchbadge/sqlx/blob/b6e127561797fe9aababa24ec640275ecb9b42af/sqlx-core/src/value.rs
// /// An owned value from the database.
// pub trait Value {
//
// }




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


    // from java R2DBC - extends readable
    // RowMetadata getMetadata();

}

/// Represents the metadata for a row of the results returned from a query.
/// Metadata for columns can be either retrieved by specifying a column name or the column index.
/// Columns indexes are 0-based.
/// Column names do not necessarily reflect the column names how they are in the underlying tables
/// but rather how columns are represented (e.g. aliased) in the result.
pub trait RowMetadata {

    /// Returns the [ColumnMetadata] for one column in this row.
    ///
    /// Arguments:
    ///
    /// * `index`: the column index starting at 0
    ///
    /// return the [ColumnMetadata] for one column in this row
    /// return index out of bounds error if [index] is out of range (negative or equals/exceeds [getColumnMetadatas().len()]
    fn get_column_metadata(index: i32) -> Result<Box<dyn ColumnMetadata>>;


    /// Returns the [ColumnMetadata] for one column in this row.
    ///
    /// Arguments:
    ///
    /// * `name`: the name of the column. Column names are case insensitive.
    /// When a get method contains several columns with same name,
    /// then the value of the first matching column will be returned.
    ///
    /// return the [ColumnMetadata] for one column in this row
    /// returns illegal argument error if [name] empty
    /// returns no such element if there is no column with the [name]
    /// NoSuchElementException   if there is no column with the {@code name}
    fn get_column_metadata_by_name<S: Into<String>>(name: S) -> Result<Box<dyn ColumnMetadata>>;


    /// Returns the [ColumnMetadata] for all columns in this row.
    fn get_column_metadatas() -> Vec<Box<dyn ColumnMetadata>>;

    /// Returns whether this object contains metadata for [column_name].
    /// Lookups are case-insensitive.
    /// Implementations may allow escape characters to enforce a particular mode of comparison
    /// when querying for presence/absence of a column.
    ///
    /// return true if this object contains metadata for [column_name]; false otherwise.
    fn contains<S: Into<String>>(column_name: S) -> bool;
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


// java R2DBC extends ReadableMetadata.
/// Represents the metadata for a column of the results returned from a query.
/// The implementation of all methods except [getName()] is optional for drivers.
/// Column metadata is optionally available as by-product of statement execution on a best-effort basis.
pub trait ColumnMetadata: ReadableMetadata {

}


pub enum Nullability {
    Nullable,
    NonNull,
    Unknown
}


/// R2DBC Data Types
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

/// Represents a set of {@code OUT} parameters returned from a stored procedure.
/// Values from out parameters can be either retrieved by specifying a parameter name or the parameter index.
/// Parameter indexes are {@code 0}-based.
///
/// Parameter names used as input to getter methods are case insensitive.
/// When a get method is called with a parameter name and several parameters have the same name,
/// then the value of the first matching parameter will be returned.
/// Parameters that are not explicitly named in the query should be referenced through parameter indexes.
///
/// For maximum portability, parameters within each {@link OutParameters} should be read in
/// left-to-right order, and each parameter should be read only once.
///
/// [#get(String)] and [#get(int)] without specifying a target type returns a suitable value representation.
/// The R2DBC specification contains a mapping table that shows default mappings between database
/// types and Java types.
/// Specifying a target type, the R2DBC driver attempts to convert the value to the target type.
///
/// A parameter is invalidated after consumption.
///
/// The number, type and characteristics of parameters are described through [OutParametersMetadata].
pub trait OutParameters: Readable {
    fn get_metadata(&self) -> Box<dyn OutParametersMetadata>;
}

/// Represents the metadata for [OUT] parameters of the results returned from a stored procedure.
/// Metadata for parameters can be either retrieved by specifying a out parameter name or
/// the out parameter index.
/// Parameter indexes are 0-based.
pub trait OutParametersMetadata {

    /// Returns the [OutParameterMetadata] for one out parameter.
    ///
    /// Arguments:
    /// * index the out parameter index starting at 0
    ///
    /// index out of bounds error if [index] is out of range (negative or equals/exceeds [getParameterMetadatas().len()])
    fn get_parameter_metadata_by_index(&self, index: u32) -> Result<Box<dyn OutParameterMetadata>>;

    /// Returns the [OutParameterMetadata] for one out parameter.
    ///
    /// Arguments:
    /// * name the name of the out parameter.  Parameter names are case insensitive.
    ///
    /// index out of bounds error if [index] is out of range (negative or equals/exceeds [getParameterMetadatas().len()])
    ///
    /// illegal argument error is name is empty
    /// no such element if there is no output parameter
    fn get_parameter_metadata_by_name(&self, name: &str) -> Result<Box<dyn OutParameterMetadata>>;

    /// Returns the [OutParameterMetadata] for all out parameters.
    fn get_parameter_metadatas(&self) -> Vec<Box<dyn OutParameterMetadata>>; // TODO: bound trait to this?
}

/// Represents the metadata for an [OUT] parameter.
/// The implementation of all methods except [#getName()]  is optional for drivers.
/// Parameter metadata is optionally available as by-product of statement execution on a best-effort basis.
pub trait OutParameterMetadata: ReadableMetadata {}

/// Represents the metadata for readable object, for example a column of the results returned from
/// a query or [OUT] parameter as result of running a stored procedure.
/// The implementation of all methods except [get_name()] is optional for drivers.
/// Metadata is optionally available as by-product of statement execution on a best-effort basis.
pub trait ReadableMetadata {
    // rust type
    fn rust_type(&self) -> &'static str;

    // type
    /// returns the database Type [TypeInfo]
    fn db_type(&self) -> dyn TypeInfo;

    /// Returns the name.
    ///
    /// The name does not necessarily reflect the names how they are in the underlying tables but
    /// rather how results are represented (e.g. aliased) in the result.
    fn get_name(&self) -> String;

    /// Returns the native type descriptor that potentially exposes more metadata.
    /// Drivers should implement this method if they can expose a driver-specific type metadata
    /// object exposing additional information.
    ///
    /// The default implementation returns [None].
    fn get_native_type_metadata(&self);

    // TODO: is this required?
    /// Returns the nullability of values.
    /// Implementation of this method is optional.
    /// The default implementation returns [Nullability::Unknown].
    fn get_nullability(&self) -> Nullability {
        Nullability::Unknown
    }

    /// Returns the precision.
    ///
    /// * For numeric data, this is the maximum precision.
    /// * For character data, this is the length in characters.
    /// * For datetime data types, this is the length in bytes required to represent the value (assuming the
    /// * maximum allowed precision of the fractional seconds component).
    /// * For binary data, this is the length in bytes.
    /// * Returns {@code null} for data types where data type size is not applicable or if the precision cannot be provided.
    ///
    /// Implementation of this method is optional.
    /// The default implementation returns [None].
    fn get_precision(&self) -> Option<u64> {
        None
    }

    /// Returns the scale.
    ///
    /// * This is the number of digits to right of the decimal point.
    /// * Returns {@code null} for data types where the scale is not applicable or if the scale cannot be provided.
    ///
    /// Implementation of this method is optional.
    /// The default implementation returns [None].
    ///
    /// the scale or [None] if the scale is not available.
    fn get_scale(&self) -> Option<u64> {
        None
    }
}

// TODO: revisit this along with docs relating to mapping of database types and rust
/// Represents a readable object, for example a set of columns or {@code OUT} parameters from a
/// database query, later on referred to as items.
/// Values can for columns or {@code OUT} parameters be either retrieved by specifying a name or the index.
/// Indexes are {@code 0}-based.
///
/// Column and [OUT] parameter names used as input to getter methods are case insensitive.
/// When a [get] method is called with a name and several items have the same name, then the value
/// of the first matching item will be returned.
/// Items that are not explicitly named in the query should be referenced through indexes.
///
/// For maximum portability, items within each [Readable] should be read in left-to-right order,
/// and each item should be read only once.
///
/// [#get(String)] and [#get(int)] without specifying a target type returns a suitable value representation.
/// The R2DBC specification contains a mapping table that shows default mappings between database
/// types and Rust types.
/// Specifying a target type, the R2DBC driver attempts to convert the value to the target type.
///
/// A item is invalidated after consumption.
pub trait Readable {

    /// Returns the value for a parameter.
    ///
    /// Arguments:
    ///
    /// * `index`: the index starting at 0
    ///
    /// return the value which can be None
    /// index out of bounds error in index is out of range (negative or equals/exceeds the number of readable objects)
    fn get<T: FromSql>(&self, index: u32) -> Result<T>;

    /// Returns the value for a parameter.
    ///
    /// Arguments:
    ///
    /// * `index`: the index starting at 0
    ///
    /// return the value which can be None
    /// index out of bounds error in index is out of range (negative or equals/exceeds the number of readable objects)
    fn get_by_name<S: Into<String>, T: FromSql>(&self, name: S) -> Result<T>;

}


// TODO: change name
/// A specialized result type representing the result of deserializing
/// a value from the database.
pub type FomSqlResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

// https://github.com/diesel-rs/diesel/blob/8a53cb7c8a09ae891df6c104d1c8a03d51ee07fc/diesel/src/deserialize.rs#L412
pub trait FromSql {

    // TODO: I'm sure this wont work and we'll need to do something more
    fn from_sql<T>(bytes: T) -> Result<Box<Self>>;

}



// * Represents a parameter to be interchanged. Parameters are typed and can define a value.
// Parameters without a value correspond with a SQL {@code NULL} value.
// * Parameters can be classified as {@link In input} or {@link Out output} parameters.
pub trait Parameter {
    // get type
    // get value
}

// Marker interface to classify a parameter as input parameter.
// Parameters that do not implement {@link Out} default to in parameters.
pub trait In {}

// Marker interface to classify a parameter as output parameter.
// Parameters can implement both, {@code In} and {@code Out} interfaces to be classified as in-out parameters.
pub trait Out {}


// Lob / Clob / Blob




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
