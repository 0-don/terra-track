mod db;
mod ip_iterator;
mod scanner;
mod scripts;
use crate::scripts::Script;
use ip_iterator::public_ips;
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
    // let res = scan("45.33.32.156").await?;
    public_ips();
    // println!("Result: {:?}", res);

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
