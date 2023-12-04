use sea_orm::DatabaseConnection;
pub mod query_root;

pub struct OrmDataloader {
    pub db: DatabaseConnection,
}
