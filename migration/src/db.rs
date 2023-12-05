use crate::Migrator;
use async_std::sync::Mutex;
use lazy_static::lazy_static;
use sea_orm_migration::sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use std::env;

lazy_static! {
    static ref MIGRATION_DONE: Mutex<bool> = Mutex::new(false);
}

pub async fn get_db_connection() -> anyhow::Result<DatabaseConnection> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");

    let mut migration_done = MIGRATION_DONE.lock().await;
    if !*migration_done {
        println!("Running migrations...");
        Migrator::up(&conn, None).await?;
        *migration_done = true;
    }

    Ok(conn)
}
