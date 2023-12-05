use crate::Migrator;
use sea_orm_migration::sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::{DbErr, MigratorTrait};
use std::env;
use std::sync::Once;

#[allow(dead_code)]
static INIT: Once = Once::new();

pub async fn get_db_connection() -> Result<DatabaseConnection, DbErr> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");

    if !INIT.is_completed() {
        println!("Running migrations...");
        let conn_for_migration = conn.clone();
        Migrator::up(&conn_for_migration, None).await?;
        INIT.call_once(|| ());
    }

    Ok(conn)
}
