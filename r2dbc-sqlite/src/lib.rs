pub mod connection;
pub mod options;
pub mod error;

use std::collections::HashMap;
use std::ops::Deref;
use rusqlite::{Rows, TransactionBehavior};
use crate::connection::SqliteConnectionMetadata;
use rusqlite::Error as RusqliteError;
use std::sync::{Arc, Mutex};
use std::rc::Rc;
use r2dbc_core::connection::{Batch, ConnectionMetadata, IsolationLevel, SQLResult, Statement, ValidationDepth};
use r2dbc_core::{DatabaseMetadata, Result, ResultSetMetaData};

// https://tedspence.com/investigating-rust-with-sqlite-53d1f9a41112
// https://www.reddit.com/r/rust/comments/dqa4t3/how_to_put_two_variables_one_borrows_from_other/
// https://bryce.fisher-fleig.org/strategies-for-returning-references-in-rust/



/// Convert a Sqlite error into an R2DBC error
fn to_r2dbc_err(e: rusqlite::Error) -> r2dbc_core::error::R2dbcErrors {
    r2dbc_core::error::R2dbcErrors::General(format!("{:?}", e))
}

// #[derive(Debug)]
// pub enum SqliteError {
//     General(String),
//     Unsupported(String),
// }
//
// impl From<RusqliteError> for SqliteError {
//     fn from(_err: ExecuteReturnedResults) -> Mishap {
//         General
//     }
// }



// const SQLITE_OPEN_READ_ONLY     = ffi::SQLITE_OPEN_READONLY;
// /// The database is opened for reading and writing if possible,
// /// or reading only if the file is write protected by the operating system.
// /// In either case the database must already exist, otherwise an error is returned.
// const SQLITE_OPEN_READ_WRITE    = ffi::SQLITE_OPEN_READWRITE;
// /// The database is created if it does not already exist
// const SQLITE_OPEN_CREATE        = ffi::SQLITE_OPEN_CREATE;
// /// The filename can be interpreted as a URI if this flag is set.
// const SQLITE_OPEN_URI           = 0x0000_0040;
// /// The database will be opened as an in-memory database.
// const SQLITE_OPEN_MEMORY        = 0x0000_0080;
// /// The new database connection will use the "multi-thread" threading mode.
// const SQLITE_OPEN_NO_MUTEX      = ffi::SQLITE_OPEN_NOMUTEX;
// /// The new database connection will use the "serialized" threading mode.
// const SQLITE_OPEN_FULL_MUTEX    = ffi::SQLITE_OPEN_FULLMUTEX;
// /// The database is opened shared cache enabled.
// const SQLITE_OPEN_SHARED_CACHE  = 0x0002_0000;
// /// The database is opened shared cache disabled.
// const SQLITE_OPEN_PRIVATE_CACHE = 0x0004_0000;
// /// The database filename is not allowed to be a symbolic link.
// const SQLITE_OPEN_NOFOLLOW = 0x0100_0000;

// these are the default flags
// OpenFlags::SQLITE_OPEN_READ_WRITE
// | OpenFlags::SQLITE_OPEN_CREATE
// | OpenFlags::SQLITE_OPEN_NO_MUTEX
// | OpenFlags::SQLITE_OPEN_URI


// in_memory
// in_memory with flags
// path
// path with flags

pub struct SqliteConnection {
    // TODO: given Transaction can reference a conn i'm not sure this is feasible
    // conn: Mutex<Option<&'conn rusqlite::Connection>>,
    // this needs to be a reference to Rusqlite::Connection otherwise we get an error like
    // move occurs because `self.conn` has type `Option<rusqlite::Connection>`, which does not implement the `Copy` trait
    // help: consider borrowing the `Option`'s content: `self.conn.as_ref()`
    //
    // because of E0106 it requires a lifetime
    conn: Option<Arc<Mutex<rusqlite::Connection>>>,
}

impl SqliteConnection {

    pub fn new(conn: rusqlite::Connection) -> Self {
        // Self { conn: Mutex::new(Some(conn)), transaction: None }
        // Self { conn: Some(conn), transaction: None }
        // Self {
        //     conn: Some(Rc::new(conn))
        // }
        Self {
            conn: Some(Arc::new(Mutex::new(conn))),
        }
    }

    fn drop(&mut self) {
        // if let Some(transaction) = self.transaction.take() {
        //     let _ = transaction.rollback();
        // }
    }
}

impl r2dbc_core::connection::Connection for SqliteConnection {
    // type Statement = SqliteStatement<'conn>;

    // TODO: result?
    fn begin_transaction(&mut self) -> Result<()> {
        // TODO: call begin_transaction_with_definition with an empty instance
        // let mut connection = self.conn.take().unwrap();
        // let mut connection = self.conn.lock().unwrap().take().unwrap();
        // connection.transaction().map_err(to_r2dbc_err);
        // self.transaction = Some(trans);
        Ok(())
    }

    // fn begin_transaction_with_definition(&mut self, definition: &dyn TransactionDefinition) {
    //     // TODO: convert definition to TransactionBehavior
    //     // let mut connection = self.conn.take().unwrap();
    //     // let mut connection = self.conn.lock().unwrap().take().unwrap();
    //     // connection.transaction_with_behavior(TransactionBehavior::Deferred);
    // }

    // https://www.reddit.com/r/rust/comments/2t8i2s/yet_another_problem_with_mutable_struct_members/
    // TODO: should return a result
    fn close(&mut self) -> Result<()> {
        // let close_result = self.conn.get_mut().unwrap().close();

        // self.conn.lock().unwrap().map(|c| c.close());
        let mut _c = self.conn.take();
        // let mut _c = self.conn.get_mut().unwrap().take();
        _c = None;

        // close_result.map_err(move |e| to_r2dbc_err(e.1))?;
        Ok(())
    }

    fn commit_transaction(&mut self) {
        // if let Some(transaction) = self.transaction.take() {
        //     let _ = transaction.commit();
        // }
    }

    fn create_batch(&mut self) -> Result<Box<dyn Batch>> {
        todo!()
    }

    // TODO: return result
    // UnsupportedOperationException if not supported
    fn create_savepoint(&mut self, name: &str) {
        // if self.transaction.is_none() {
        //     // return error
        // }

        // let sp = self.conn.savepoint_with_name(name)?;
        // let savepoint = self.transaction.unwrap().savepoint_with_name(name)?;
    }

    // fn create_statement(&mut self, sql: &str) -> Result<Box<Self::Statement>> {
    fn create_statement(&mut self, sql: &str) -> Result<Box<dyn Statement<'_> + '_>> {
        // let mut c = self.conn.take();
        // let stmt = c.unwrap()
        //     .prepare(sql)
        //     .map_err(to_r2dbc_err)?;

        // let c: &'conn rusqlite::Connection = self.conn.unwrap();

        // let stmt: rusqlite::Statement = self.conn.unwrap()
        //     .prepare(sql)
        //     .map_err(to_r2dbc_err)?;

        // let stmt = self.conn.get_mut().unwrap().take().unwrap()
        //     .prepare(sql)
        //     .map_err(to_r2dbc_err)?;

        // let stmt = self.conn.lock().unwrap().take().unwrap()
        //     .prepare(sql)
        //     .map_err(to_r2dbc_err)?;

        // let stmt = self.conn
        //     .as_ref()
        //     .unwrap()
        //     .clone()
        //     .deref()
        //     .lock()
        //     .unwrap()
        //     .prepare(sql)
        //     .map_err(to_r2dbc_err)?;
        //
        // Ok(Box::new(SqliteStatement {
        //     stmt,
        // }))

        todo!()
    }

    fn is_auto_commit(&mut self) -> bool {
        let connection = self.conn.take().unwrap();
        // let connection = self.conn.lock().unwrap().take().unwrap();
        connection.clone().deref().lock().unwrap().is_autocommit()
    }

    fn metadata(&mut self) -> Result<Box<dyn ConnectionMetadata>> {
        todo!()
    }

    fn transaction_isolation_level(&mut self) -> IsolationLevel {
        todo!()
    }

    fn release_savepoint(&mut self, name: &str) {
        todo!()
        // do we need to keep savepoint? I dont see rusqlite giving us an option to get a savepoint

    }

    /// This is equivalent to `Transaction`'s `Drop` implementation, but provides any error
    /// encountered to the caller.
    fn rollback_transaction(&mut self) {
        todo!()
    }

    fn rollback_transaction_to_savepoint(&mut self, name: String) {
        todo!()
    }

    fn auto_commit(&mut self, commit: bool) {
        todo!()
        // The sqlite3_get_autocommit() interface returns non-zero or zero if the given database
        // connection is or is not in autocommit mode, respectively.
        // Autocommit mode is on by default.
        // Autocommit mode is disabled by a BEGIN statement. Autocommit mode is re-enabled by a
        // COMMIT or ROLLBACK.
    }

    fn set_transaction_isolation_level(&mut self, isolation_level: IsolationLevel) {
        // Error::Unsupported(String::from(
        //     "Except in the case of shared cache database connections with PRAGMA read_uncommitted \
        //     turned on, all transactions in SQLite show \"serializable\" isolation. \
        //     SQLite implements serializable transactions by actually serializing the writes."
        // ))
    }

    fn validate(&mut self, depth: ValidationDepth) -> bool {
        todo!()
    }
}

// impl Drop for SqliteConnection {
//     fn drop(&mut self) {
//         let _ = self.close();
//     }
// }

// TODO: Do we need this? Can we just use CallableStatement/PreparedStatement
pub struct SqliteStatement<'a> {
    stmt: rusqlite::Statement<'a>,
}

impl r2dbc_core::connection::Statement<'_> for SqliteStatement<'_> {
    fn add(&mut self) -> &mut Self where Self: Sized {
        todo!()
    }

    fn bind_index<T>(&mut self, index: u32, value: T) -> &mut Self where Self: Sized {
        todo!()
    }

    fn bind_name<T>(&mut self, name: &str, value: T) -> &mut Self where Self: Sized {
        todo!()
    }

    fn bind_null_index(&mut self, index: u32) -> &mut Self where Self: Sized {
        todo!()
    }

    fn bind_null_name(&mut self, name: &str) -> &mut Self where Self: Sized {
        todo!()
    }

    fn execute<T: SQLResult>(&self) -> Result<T> where Self: Sized {
        todo!()
    }

    fn return_generated_values(&mut self, columns: &[&str]) -> &mut Self where Self: Sized {
        todo!()
    }

    fn fetch_size(&mut self, rows: u32) -> &mut Self where Self: Sized {
        todo!()
    }
}

struct SqliteResultSet<'stmt> {
    rows: Rows<'stmt>,
}

struct SqliteDatabaseMetadata {

}

impl DatabaseMetadata for SqliteDatabaseMetadata {

}

impl<'stmt> r2dbc_core::ResultSet for SqliteResultSet<'stmt> {
    fn meta_data(&self) -> Result<Box<dyn ResultSetMetaData>> {
        todo!()
    }

    fn next(&mut self) -> bool {
        todo!()
    }

    fn get_bool(&self, i: u64) -> Result<Option<bool>> {
        todo!()
    }

    fn get_i8(&self, i: u64) -> Result<Option<i8>> {
        todo!()
    }

    fn get_i16(&self, i: u64) -> Result<Option<i16>> {
        todo!()
    }

    fn get_i32(&self, i: u64) -> Result<Option<i32>> {
        todo!()
    }

    fn get_i64(&self, i: u64) -> Result<Option<i64>> {
        todo!()
    }

    fn get_f32(&self, i: u64) -> Result<Option<f32>> {
        todo!()
    }

    fn get_f64(&self, i: u64) -> Result<Option<f64>> {
        todo!()
    }

    fn get_string(&self, i: u64) -> Result<Option<String>> {
        todo!()
    }

    fn get_bytes(&self, i: u64) -> Result<Option<Vec<u8>>> {
        todo!()
    }
}

fn to_r2dbc_type(t: Option<&str>) -> r2dbc_core::DataType {
    //TODO implement for real
    match t {
        Some("INT") => r2dbc_core::DataType::Integer,
        _ => r2dbc_core::DataType::Utf8,
    }
}

struct Values<'a>(&'a [r2dbc_core::Value]);
struct ValuesIter<'a>(std::slice::Iter<'a, r2dbc_core::Value>);

impl<'a> IntoIterator for &'a Values<'a> {
    type Item = &'a dyn rusqlite::types::ToSql;
    type IntoIter = ValuesIter<'a>;

    fn into_iter(self) -> ValuesIter<'a> {
        ValuesIter(self.0.iter())
    }
}
impl<'a> Iterator for ValuesIter<'a> {
    type Item = &'a dyn rusqlite::types::ToSql;

    fn next(&mut self) -> Option<&'a dyn rusqlite::types::ToSql> {
        self.0.next().map(|v| match v {
            r2dbc_core::Value::String(ref s) => s as &dyn rusqlite::types::ToSql,
            r2dbc_core::Value::Int32(ref n) => n as &dyn rusqlite::types::ToSql,
            r2dbc_core::Value::UInt32(ref n) => n as &dyn rusqlite::types::ToSql,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{collections::HashMap, sync::Arc};
    use crate::options::SqliteConnectOptions;

    // // low-level, Executor trait
    // conn.execute("BEGIN").await?; // unprepared, simple query
    // conn.execute(sqlx::query("DELETE FROM table")).await?; // prepared, cached query

    // sqlx::query("DELETE FROM table").execute(&mut conn).await?;
    // sqlx::query("DELETE FROM table").execute(&pool).await?;

    // let mut rows = sqlx::query("SELECT * FROM users WHERE email = ?")
    // .bind(email)
    // .fetch(&mut conn);
    //
    // while let Some(row) = rows.try_next().await? {
    //     // map the row into a user-defined domain type
    //     let email: &str = row.try_get("email")?;
    // }

//     let mut stream = sqlx::query("SELECT * FROM users")
//     .map(|row: PgRow| {
// // map the row into a user-defined domain type
// })
//     .fetch(&mut conn);
//     #[derive(sqlx::FromRow)]
//     struct User { name: String, id: i64 }
//
//     let mut stream = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = ? OR name = ?")
//     .bind(user_email)
//     .bind(user_name)
//     .fetch(&mut conn);



    #[test]
    fn execute_query() -> r2dbc_core::Result<()> {
        // let mut connection = SqliteConnectOptions::new().connect().await?;
        // let stmt = connection.create_statement("SELECT 1").unwrap();
        // let mut rs = stmt.execute();
        //
        // while rs.next() {
        //     println!("{:?}", rs.get_string(1));
        // }

        Ok(())
    }

    // #[test]
    // fn execute_query() -> r2dbc::Result<()> {
    //     let driver: Arc<dyn r2dbc::Driver> = Arc::new(SqliteDriver::new());
    //     let url = "";
    //     let mut conn = driver.connect(url, HashMap::new())?;
    //     execute(&mut *conn, "DROP TABLE IF EXISTS test", &vec![])?;
    //     execute(&mut *conn, "CREATE TABLE test (a INT NOT NULL)", &vec![])?;
    //     execute(
    //         &mut *conn,
    //         "INSERT INTO test (a) VALUES (?)",
    //         &vec![r2dbc::Value::Int32(123)],
    //     )?;
    //
    //     let mut stmt = conn.prepare("SELECT a FROM test")?;
    //     let mut rs = stmt.execute_query(&vec![])?;
    //
    //     let meta = rs.meta_data()?;
    //     assert_eq!(1, meta.num_columns());
    //     assert_eq!("a".to_owned(), meta.column_name(0));
    //     assert_eq!(DataType::Integer, meta.column_type(0));
    //
    //     assert!(rs.next());
    //     assert_eq!(Some(123), rs.get_i32(0)?);
    //     assert!(!rs.next());
    //
    //     Ok(())
    // }

    // fn execute(
    //     conn: &mut dyn Connection,
    //     sql: &str,
    //     values: &Vec<r2dbc::Value>,
    // ) -> r2dbc::Result<u64> {
    //     println!("Executing '{}' with {} params", sql, values.len());
    //     let mut stmt = conn.prepare(sql)?;
    //     stmt.execute_update(values)
    // }
}
