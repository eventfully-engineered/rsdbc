use r2dbc::ConnectionMetadata;

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