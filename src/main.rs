use scanner::db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    db::get_db_connection().await?;

    std::thread::spawn(|| scanner::main());
    let _ = std::thread::spawn(|| api::main()).join();

    Ok(())
}
