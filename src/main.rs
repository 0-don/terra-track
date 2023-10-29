mod ip_iterator;
mod scanner;
mod scripts;
mod types;
mod utils;
use crate::utils::scan;
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().expect(".env file not found");

    // let open_scan = scan_batch_service::Query::find_open_scans().await?;

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
