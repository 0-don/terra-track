#![allow(dead_code)]
#![allow(while_true)]
pub mod db;
pub mod mapper;
pub mod parser;
pub mod scanner;
pub mod service;
pub mod types;
pub mod utils;
use crate::{parser::parse_nmap_results, service::ip_main_e::ip_main_q};
use chrono::Duration;
use dotenvy::dotenv;
use entity::scan_batch;
use migration::sea_orm::Set;
use scanner::{ip_iterator::Ipv4Iter, nmap_scanner::NmapScanner, port_scanner::PortScanner};
use service::{
    ip_main_e::ip_main_m,
    scan_batch_e::{scan_batch_m, scan_batch_q},
};
use std::{fs::remove_dir_all, net::IpAddr};
use utils::date;

pub static VALUE: &str = "value";
pub static ELEM: &str = "elem";
pub static TABLE: &str = "table";
pub static BATCH_SIZE: i32 = 20;

#[tokio::main]
async fn start() -> anyhow::Result<()> {
    dotenv().ok();
    // reset(false).await?;
    // single_scan("1.0.15.178").await?;
    while true {
        if !cfg!(debug_assertions) {
            loop_scan().await?;
        }
    }

    Ok(())
}

/// Resets the scan batches and IP main data.
/// Optionally deletes the output folder in debug configuration.
async fn reset(delete_folder: bool) -> anyhow::Result<()> {
    scan_batch_m::Mutation::delete_all_scan_batch().await?;
    ip_main_m::Mutation::delete_all_ip_main().await?;
    if cfg!(debug_assertions) && delete_folder {
        remove_dir_all("./output").ok(); // Changed to handle errors gracefully
    }

    Ok(())
}

/// Performs a loop scan based on the next scan batch.
async fn loop_scan() -> anyhow::Result<()> {
    let scan = scan_batch_q::Query::next_scan_batch().await?;
    let mut ip_iter = Ipv4Iter::batched(&scan.ip, scan.batch_size);

    while let Some(ip) = ip_iter.next() {
        single_scan(&ip.to_string()).await?;
    }

    scan_batch_m::Mutation::update_scan_batch(scan_batch::ActiveModel {
        id: Set(scan.id),
        end: Set(Some(date(Duration::zero()))),
        ..Default::default()
    })
    .await?;

    Ok(())
}

/// Logs the given message with a timestamp.
macro_rules! printlog {
    ($($arg:tt)*) => {{
        let now = chrono::Local::now();
        println!("{}.{:03}: {}", now.format("%Y-%m-%d %H:%M:%S"), now.timestamp_subsec_millis(), format!($($arg)*));
    }};
}

/// Performs a single scan on the specified IP address.
async fn single_scan(ip_str: &str) -> anyhow::Result<()> {
    let ip: IpAddr = ip_str.parse()?;
    printlog!("Scanning IP: {}", ip);

    if let Some(_) =
        ip_main_q::Query::find_ip_main_by_ip_older_then(ip_str, Some(date(Duration::days(365))))
            .await?
    {
        printlog!("IP already scanned: {}", ip);
        return Ok(());
    }

    let mut nmap = NmapScanner::new(ip, vec![]).parse_nmap_xml();
    if nmap.is_err() {
        let scanned_ports = PortScanner::new(ip).run().await?;
        printlog!("Open ports: {:?}", scanned_ports);

        if scanned_ports.is_empty() {
            printlog!("No open ports found: {}", ip);
            return Ok(());
        }

        nmap = NmapScanner::new(ip, scanned_ports).run();
    }

    if let Ok(nmap_results) = nmap {
        parse_nmap_results(&nmap_results).await?;
    }

    Ok(())
}

/// Deletes the latest scan batch and IP main data.
async fn delete_last() -> anyhow::Result<()> {
    scan_batch_m::Mutation::delete_latest_scan_batch().await?;
    ip_main_m::Mutation::delete_latest_ip_main().await?;
    Ok(())
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}

#[macro_export]
macro_rules! printlog {
    ($($arg:tt)*) => {
        {
            use chrono::{Local, DateTime};
            let now: DateTime<Local> = Local::now();
            let millis = now.timestamp_subsec_millis();
            println!("{}.{:03}: {}", now.format("%Y-%m-%d %H:%M:%S"), millis, format!($($arg)*));
        }
    };
}
