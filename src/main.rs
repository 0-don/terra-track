use dotenvy::dotenv;
use service::models::scan_batch_service;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().expect(".env file not found");

    let open_scan = scan_batch_service::Query::next_scan_batch().await?;

    printlog!("Open scan: {:?}", open_scan);

    // let cursor = "0.0.0.0"; // Start from this IP
    // let mut ip_iter = Ipv4Iter::new(cursor);

    // printlog!("Starting scan...");

    // while let Some(ip) = ip_iter.next() {
    //     println!("{}", ip);
    // }

    // printlog!("Scan complete");

    // let res = scan("45.33.32.156").await?;
    // println!("Result: {:?}", res);
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

// //ignore linting
// #[allow(dead_code)]
// pub async fn scan(ip: &'static str) -> anyhow::Result<NmapXML> {
//     let ip: IpAddr = ip.parse()?;
//     let ports = Scanner::new(ip).run().await?;

//     println!("IP {:?} Open ports: {:?}", ip.to_string(), ports);

//     let script = Script::new(ip, ports);
//     let result = script.run();
//     if let Ok(result) = result {
//         return Ok(result);
//     }

//     Err(result.err().unwrap())
// }
