use crate::error::R2dbcErrors;

pub mod error;
pub mod connection;

/// R2DBC Result type
pub type Result<T> = std::result::Result<T, R2dbcErrors>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
