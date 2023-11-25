use chrono::Duration;
use dotenvy::dotenv;
use migration::sea_orm::Set;
use scanner::{ip_iterator::Ipv4Iter, nmap_scanner::NmapScanner};
use service::{
    entity::{
        ip_main_m::{ip_main_m, ip_main_q},
        scan_batch::{scan_batch_m, scan_batch_q},
    },
    parser::parse_nmap_results,
    utils::date,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().expect(".env file not found");

    scan_batch_m::Mutation::delete_all_scan_batch().await?;
    let scan = scan_batch_q::Query::next_scan_batch().await?;
    ip_main_m::Mutation::delete_all_ip_main().await?;
    // if cfg!(debug_assertions) {
    //     let _ = remove_dir_all("./output");
    // }

    let mut ip_iter = Ipv4Iter::batched(&scan.ip, scan.batch_size);
    while let Some(ip) = ip_iter.next() {
        printlog!("Scanning IP: {}", ip);

        let ip_main = ip_main_q::Query::find_ip_main_by_ip_older_then(
            &ip.to_string(),
            Some(date(Duration::days(365))),
        )
        .await?;

        if ip_main.is_some() {
            printlog!("IP already scanned: {}", ip);
            continue;
        }

        // remove folder recursively

        // let ports = Scanner::new(ip.into()).run().await?;
        let ports = vec![];
        printlog!("Open ports: {:?}", ports);
        let result = NmapScanner::new(ip.into(), ports).run();

        if let Ok(nmap) = result {
            parse_nmap_results(&nmap).await?;
        }
    }

    // let ports = Scanner::new("45.33.32.156".parse()?).run().await?;
    // printlog!("Open ports: {:?}", ports);
    // let result = Script::new("45.33.32.156".parse()?, ports).run();

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
