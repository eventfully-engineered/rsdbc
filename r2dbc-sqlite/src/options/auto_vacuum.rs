// From SQLx - https://github.com/launchbadge/sqlx/blob/master/sqlx-core/src/sqlite/options/auto_vacuum.rs

#[derive(Debug, Clone)]
pub enum SqliteAutoVacuum {
    None,
    Full,
    Incremental,
}

// TODO: display trait instead?
impl SqliteAutoVacuum {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            SqliteAutoVacuum::None => "NONE",
            SqliteAutoVacuum::Full => "FULL",
            SqliteAutoVacuum::Incremental => "INCREMENTAL",
        }
    }
}

impl Default for SqliteAutoVacuum {
    fn default() -> Self {
        SqliteAutoVacuum::None
    }
}