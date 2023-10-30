use dotenvy::dotenv;
use scanner::{ip_iterator::Ipv4Iter, scanner::Scanner};
use service::models::scan_batch_service;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().expect(".env file not found");

    let scan = scan_batch_service::Query::next_scan_batch().await?;

    printlog!("Open scan: {:?}", scan);

    let mut ip_iter = Ipv4Iter::batched(&scan.ip, scan.batch_size);
    while let Some(ip) = ip_iter.next() {
        printlog!("Scanning IP: {}", ip);

        let res = Scanner::new(ip.into()).run().await?;
        println!("Result: {:?}", res);
        // let res = scan(ip.to_string().as_str()).await?;
        // println!("Result: {:?}", res);
    }

    // printlog!("Scan complete");

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
