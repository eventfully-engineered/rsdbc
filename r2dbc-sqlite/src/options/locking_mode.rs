// From SQLx - https://github.com/launchbadge/sqlx/blob/master/sqlx-core/src/sqlite/options/locking_mode.rs

#[derive(Debug, Clone)]
pub enum SqliteLockingMode {
    Normal,
    Exclusive,
}

impl SqliteLockingMode {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            SqliteLockingMode::Normal => "NORMAL",
            SqliteLockingMode::Exclusive => "EXCLUSIVE",
        }
    }
}

impl Default for SqliteLockingMode {
    fn default() -> Self {
        SqliteLockingMode::Normal
    }
}