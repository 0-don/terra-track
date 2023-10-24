mod db;
mod ip_iterator;
mod scanner;
mod scripts;
use crate::scripts::Script;
use db::connect_db;
use ip_iterator::Ipv4Iter;
use nmap_xml_parser::NmapResults;
use scanner::Scanner;
use std::{fs::File, io::Read, net::IpAddr};

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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Starting main function...");

    println!("Connecting to DB...");
    connect_db().await?;
    println!("DB connected.");

    // let res = scan("45.33.32.156").await?;
    // public_ips();
    // println!("Result: {:?}", res);

    let cursor = "0.0.0.0"; // Start from this IP
    let offset = 100;

    println!("Initializing IP iterator...");
    let mut ip_iter = Ipv4Iter::new(cursor, offset);

    println!("Printing IPs...");
    // Print the first 10 IP addresses
    for _ in 0..10 {
        if let Some(ip) = ip_iter.next() {
            println!("{}", ip);
        }
    }

    println!("Main function completed.");
    Ok(())
}

pub async fn scan(ip: &'static str) -> anyhow::Result<NmapResults> {
    let ip: IpAddr = ip.parse()?;
    let ports = Scanner::new(ip).run().await?;

    println!("IP {:?} Open ports: {:?}", ip.to_string(), ports);

    let script = Script::new(ip, ports);
    let result = script.run();
    if let Ok(result) = result {
        println!("Script result: {:?}", result);

        let mut file = File::open(format!("./{}.xml", ip.to_string())).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let nmap: NmapResults = NmapResults::parse(&contents).unwrap();

        return Ok(nmap);
    }

    Err(result.err().unwrap())
}
