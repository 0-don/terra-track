use chrono::Duration;
use dotenvy::dotenv;
use migration::sea_orm::Set;
use scanner::{ip_iterator::Ipv4Iter, nmap_scanner::NmapScanner, port_scanner::PortScanner};
use service::{
    entity::{
        ip_main_e::{ip_main_m, ip_main_q},
        scan_batch_e::{scan_batch_m, scan_batch_q},
    },
    parser::parse_nmap_results,
    utils::date,
};
use std::fs::remove_dir_all;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().expect(".env file not found");
    // reset().await?;
    // delete_last().await?;
    loop_scan().await?;

    Ok(())
}

#[allow(dead_code)]
async fn reset() -> anyhow::Result<()> {
    scan_batch_m::Mutation::delete_all_scan_batch().await?;
    ip_main_m::Mutation::delete_all_ip_main().await?;
    if cfg!(debug_assertions) {
        let _ = remove_dir_all("./output");
    }

    Ok(())
}

#[allow(dead_code)]
async fn loop_scan() -> anyhow::Result<()> {
    let scan = scan_batch_q::Query::next_scan_batch().await?;
    let mut ip_iter = Ipv4Iter::batched(&scan.ip, scan.batch_size);
    while let Some(ip) = ip_iter.next() {
        printlog!("Scanning IP: {}", ip);

        let mut result = NmapScanner::new(ip.into(), vec![]).parse_nmap_xml();

        let ip_main = ip_main_q::Query::find_ip_main_by_ip_older_then(
            &ip.to_string(),
            Some(date(Duration::days(365))),
        )
        .await?;

        if ip_main.is_some() {
            printlog!("IP already scanned: {}", ip);
            continue;
        }
        #[allow(unused_variables, unused_mut, unused_assignments)]
        let mut ports: Vec<u16> = vec![];
        if result.is_err() {
            ports = PortScanner::new(ip.into()).run().await?;
            printlog!("Open ports: {:?}", ports);
            result = NmapScanner::new(ip.into(), ports).run();
        }

        if let Ok(nmap) = result {
            parse_nmap_results(&nmap).await?;
        }
    }
    scan_batch_m::Mutation::update_scan_batch(entity::scan_batch::ActiveModel {
        id: Set(scan.id),
        end: Set(Some(date(Duration::zero()))),
        ..Default::default()
    })
    .await?;

    Ok(())
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

#[allow(dead_code)]
async fn single_scan(str: &str) -> anyhow::Result<()> {
    let ports = PortScanner::new(str.parse()?).run().await?;
    printlog!("Open ports: {:?}", ports);
    let result = NmapScanner::new(str.parse()?, ports).run();

    if let Ok(nmap) = result {
        parse_nmap_results(&nmap).await?;
    }

    Ok(())
}

#[allow(dead_code)]
async fn delete_last() -> anyhow::Result<()> {
    scan_batch_m::Mutation::delete_latest_scan_batch().await?;
    ip_main_m::Mutation::delete_latest_ip_main().await?;
    Ok(())
}
