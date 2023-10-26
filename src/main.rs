mod db;
mod ip_iterator;
mod scanner;
mod scripts;
mod types;
mod utils;
use db::get_db_connection;
use dotenvy::dotenv;

use crate::utils::scan;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().expect(".env file not found");
    get_db_connection().await?;
    // let cursor = "0.0.0.0"; // Start from this IP
    // let mut ip_iter = Ipv4Iter::new(cursor);

    // printlog!("Starting scan...");

    // while let Some(ip) = ip_iter.next() {
    //     println!("{}", ip);
    // }

    // printlog!("Scan complete");

    let res = scan("45.33.32.156").await?;
    println!("Result: {:?}", res);
    Ok(())
}
