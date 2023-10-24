use std::{fs::File, io::Read, net::IpAddr};

use nmap_xml_parser::NmapResults;

use crate::{scanner::Scanner, scripts::Script};

pub mod macros;

// let res = scan("45.33.32.156").await?;
// println!("Result: {:?}", res);

//ignore linting
#[allow(dead_code)]
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
