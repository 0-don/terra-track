mod db;
mod ip_iterator;
mod scanner;
mod scripts;
mod utils;
use db::connect_db;
use dotenvy::dotenv;
use ip_iterator::Ipv4Iter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().expect(".env file not found");
    connect_db().await?;

    let cursor = "0.0.0.0"; // Start from this IP
    let mut ip_iter = Ipv4Iter::new(cursor);

    for _ in 0..10 {
        if let Some(ip) = ip_iter.next() {
            println!("{}", ip);
        }
    }

    Ok(())
}
