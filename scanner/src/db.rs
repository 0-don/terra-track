use migration::{
    sea_orm::{Database, DatabaseConnection},
    Migrator, MigratorTrait,
};
use std::env;
use std::sync::Once;
use tokio::{runtime::Runtime, task};

#[allow(dead_code)]
static INIT: Once = Once::new();

pub async fn get_db_connection() -> anyhow::Result<DatabaseConnection> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");

    INIT.call_once(|| {
        println!("Running migrations...");
        let conn_for_migration = conn.clone();
        task::spawn_blocking(|| {
            Runtime::new().unwrap().block_on(async move {
                Migrator::up(&conn_for_migration, None).await.unwrap();
            })
        });
    });

    Ok(conn)
}
