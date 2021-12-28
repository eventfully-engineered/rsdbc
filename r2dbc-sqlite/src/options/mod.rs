// From SQLx - https://github.com/launchbadge/sqlx/blob/master/sqlx-core/src/sqlite/options/mod.rs

mod auto_vacuum;
mod journal_mode;
mod locking_mode;
mod synchronous;
mod parse;
mod mode;

// TODO: add log settings
// use crate::connection::LogSettings;
pub use auto_vacuum::SqliteAutoVacuum;
pub use journal_mode::SqliteJournalMode;
pub use locking_mode::SqliteLockingMode;
pub use synchronous::SqliteSynchronous;

use std::{borrow::Cow, time::Duration};
use std::future::Future;
use std::path::Path;
use std::pin::Pin;
use crate::{SqliteConnection, to_r2dbc_err};
use crate::Result;
use futures::future::BoxFuture;
use rusqlite::{Connection, OpenFlags};
use std::sync::{Arc, Mutex};
use rusqlite::params;
use std::rc::Rc;
use r2dbc_core::connection::{ConnectionFactory, ConnectionFactoryMetadata, ConnectionFactoryOptions};
use r2dbc_core::error::R2dbcErrors;

// // TODO:
// // - ^ the trait `From<rusqlite::Error>` is not implemented for `r2dbc::Error`
// impl From<RusqliteError> for Error {
//     fn from(err: rusqlite::Error) -> Self {
//         Error::General(err.to_string())
//     }
// }


// TODO: rename to SqliteConnectionConfiguration?
#[derive(Clone, Debug)]
pub struct SqliteConnectOptions {
    pub(crate) filename: Cow<'static, Path>,
    pub(crate) in_memory: bool,
    pub(crate) read_only: bool,
    pub(crate) create_if_missing: bool,
    pub(crate) journal_mode: SqliteJournalMode,
    pub(crate) locking_mode: SqliteLockingMode,
    pub(crate) foreign_keys: bool,
    pub(crate) shared_cache: bool,
    pub(crate) statement_cache_capacity: usize,
    pub(crate) busy_timeout: Duration,
    // pub(crate) log_settings: LogSettings,
    pub(crate) synchronous: SqliteSynchronous,
    pub(crate) auto_vacuum: SqliteAutoVacuum,
}

// TODO: document...see new
impl Default for SqliteConnectOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl SqliteConnectOptions {

    // TODO: document these options
    pub fn new() -> Self {
        Self {
            filename: Cow::Borrowed(Path::new(":memory:")),
            in_memory: false,
            read_only: false,
            create_if_missing: false,
            foreign_keys: true,
            shared_cache: false,
            statement_cache_capacity: 100,
            journal_mode: Default::default(),
            locking_mode: Default::default(),
            busy_timeout: Duration::from_secs(5),
            // log_settings: Default::default(),
            synchronous: Default::default(),
            auto_vacuum: Default::default(),
        }
    }

    /// Sets the name of the database file.
    pub fn filename(mut self, filename: impl AsRef<Path>) -> Self {
        self.filename = Cow::Owned(filename.as_ref().to_owned());
        self
    }

    /// Set the enforcement of [foreign key constraints](https://www.sqlite.org/pragma.html#pragma_foreign_keys).
    ///
    /// By default, this is enabled.
    pub fn foreign_keys(mut self, on: bool) -> Self {
        self.foreign_keys = on;
        self
    }

    /// Sets the [journal mode](https://www.sqlite.org/pragma.html#pragma_journal_mode) for the database connection.
    ///
    /// The default journal mode is WAL. For most use cases this can be significantly faster but
    /// there are [disadvantages](https://www.sqlite.org/wal.html).
    pub fn journal_mode(mut self, mode: SqliteJournalMode) -> Self {
        self.journal_mode = mode;
        self
    }

    /// Sets the [locking mode](https://www.sqlite.org/pragma.html#pragma_locking_mode) for the database connection.
    ///
    /// The default locking mode is NORMAL.
    pub fn locking_mode(mut self, mode: SqliteLockingMode) -> Self {
        self.locking_mode = mode;
        self
    }

    /// Sets the [access mode](https://www.sqlite.org/c3ref/open.html) to open the database
    /// for read-only access.
    pub fn read_only(mut self, read_only: bool) -> Self {
        self.read_only = read_only;
        self
    }

    /// Sets the [access mode](https://www.sqlite.org/c3ref/open.html) to create the database file
    /// if the file does not exist.
    ///
    /// By default, a new file **will not be** created if one is not found.
    pub fn create_if_missing(mut self, create: bool) -> Self {
        self.create_if_missing = create;
        self
    }

    /// Sets the capacity of the connection's statement cache in a number of stored
    /// distinct statements. Caching is handled using LRU, meaning when the
    /// amount of queries hits the defined limit, the oldest statement will get
    /// dropped.
    ///
    /// The default cache capacity is 100 statements.
    pub fn statement_cache_capacity(mut self, capacity: usize) -> Self {
        self.statement_cache_capacity = capacity;
        self
    }

    /// Sets a timeout value to wait when the database is locked, before
    /// returning a busy timeout error.
    ///
    /// The default busy timeout is 5 seconds.
    pub fn busy_timeout(mut self, timeout: Duration) -> Self {
        self.busy_timeout = timeout;
        self
    }

    /// Sets the [synchronous](https://www.sqlite.org/pragma.html#pragma_synchronous) setting for
    /// the database connection.
    ///
    /// The default synchronous settings is FULL. However, if durability is not a concern,
    /// then NORMAL is normally all one needs in WAL mode.
    pub fn synchronous(mut self, synchronous: SqliteSynchronous) -> Self {
        self.synchronous = synchronous;
        self
    }

    /// Sets the [auto_vacuum](https://www.sqlite.org/pragma.html#pragma_auto_vacuum) setting for
    /// the database connection.
    ///
    /// The default auto_vacuum setting is NONE.
    pub fn auto_vacuum(mut self, auto_vacuum: SqliteAutoVacuum) -> Self {
        self.auto_vacuum = auto_vacuum;
        self
    }

    /// Set the [`SQLITE_OPEN_SHAREDCACHE` flag](https://sqlite.org/sharedcache.html).
    ///
    /// By default, this is disabled.
    pub fn shared_cache(mut self, on: bool) -> Self {
        self.shared_cache = on;
        self
    }
}

impl ConnectionFactory for SqliteConnectOptions {
    // fn connect(&self) -> BoxFuture<'_, Result<Box<SqliteConnection>>>
    fn connect(&self) -> Pin<Box<dyn Future<Output = Result<Box<(dyn r2dbc_core::connection::Connection + 'static)>>> + Send>>
    {
        todo!()
        // Box::pin(async move {
        //     let mut flags = OpenFlags::SQLITE_OPEN_NO_MUTEX;
        //
        //     flags |= if self.read_only {
        //         OpenFlags::SQLITE_OPEN_READ_ONLY
        //     } else if self.create_if_missing {
        //         OpenFlags::SQLITE_OPEN_CREATE | OpenFlags::SQLITE_OPEN_READ_WRITE
        //     } else {
        //         OpenFlags::SQLITE_OPEN_READ_WRITE
        //     };
        //
        //     if self.in_memory {
        //         flags |= OpenFlags::SQLITE_OPEN_MEMORY;
        //     }
        //
        //     flags |= if self.shared_cache {
        //         OpenFlags::SQLITE_OPEN_SHARED_CACHE
        //     } else {
        //         OpenFlags::SQLITE_OPEN_PRIVATE_CACHE
        //     };
        //
        //     let conn =
        //         rusqlite::Connection::open_with_flags(self.filename.to_path_buf(), flags)
        //             .map_err(to_r2dbc_err)?;
        //
        //     conn.busy_timeout(self.busy_timeout);
        //
        //     // execute pragma
        //     let init = format!(
        //         "PRAGMA locking_mode = {}; PRAGMA journal_mode = {}; PRAGMA foreign_keys = {}; PRAGMA synchronous = {}; PRAGMA auto_vacuum = {}",
        //         self.locking_mode.as_str(),
        //         self.journal_mode.as_str(),
        //         if self.foreign_keys { "ON" } else { "OFF" },
        //         self.synchronous.as_str(),
        //         self.auto_vacuum.as_str(),
        //     );
        //     conn.execute(init.as_str(), params![]).map_err(to_r2dbc_err)?;
        //
        //     // // TODO: make this better
        //     // Ok(Box::new(SqliteConnection {
        //     //     // conn: Mutex::new(Some(&conn)),
        //     //     conn: Some(Arc::new(Mutex::new(conn))),
        //     // }) as Box<(dyn r2dbc_core::connection::Connection + 'static)>)
        //
        //     todo!()
        //
        //
        //     // let mut conn = establish(self).await?;
        //     //
        //     // // send an initial sql statement comprised of options
        //     // //
        //     // // Note that locking_mode should be set before journal_mode; see
        //     // // https://www.sqlite.org/wal.html#use_of_wal_without_shared_memory .
        //     // let init = format!(
        //     //     "PRAGMA locking_mode = {}; PRAGMA journal_mode = {}; PRAGMA foreign_keys = {}; PRAGMA synchronous = {}; PRAGMA auto_vacuum = {}",
        //     //     self.locking_mode.as_str(),
        //     //     self.journal_mode.as_str(),
        //     //     if self.foreign_keys { "ON" } else { "OFF" },
        //     //     self.synchronous.as_str(),
        //     //     self.auto_vacuum.as_str(),
        //     // );
        //     //
        //     // conn.execute(&*init).await?;
        //     //
        //     // Ok(conn)
        // })
    }

    // TODO: use SQLite Connection Factory Metadata?
    fn get_metadata(&self) -> Box<dyn ConnectionFactoryMetadata> {
        todo!()
    }
}
