// From SQLx - https://github.com/launchbadge/sqlx/blob/master/sqlx-core/src/sqlite/options/synchronous.rs

#[derive(Debug, Clone)]
pub enum SqliteSynchronous {
    Off,
    Normal,
    Full,
    Extra,
}

impl SqliteSynchronous {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            SqliteSynchronous::Off => "OFF",
            SqliteSynchronous::Normal => "NORMAL",
            SqliteSynchronous::Full => "FULL",
            SqliteSynchronous::Extra => "EXTRA",
        }
    }
}

impl Default for SqliteSynchronous {
    fn default() -> Self {
        SqliteSynchronous::Full
    }
}