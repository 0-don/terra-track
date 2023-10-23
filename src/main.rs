mod scanner;
mod scripts;
use crate::scripts::Script;
use nmap_xml_parser::NmapResults;
use scanner::Scanner;
use std::{fs::File, io::Read, net::IpAddr};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let ip: IpAddr = "45.33.32.156".parse()?;
    let ports = Scanner::new(ip).run().await?;

    println!("IP {:?} Open ports: {:?}", ip.to_string(), ports);

    let script = Script::new(ip, ports);
    let result = script.run();
    if let Ok(result) = result {
        println!("Script result: {:?}", result);

        // read file as string ./nmap.xml

        let mut file = File::open(format!("./{}.xml", ip.to_string())).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let nmap: NmapResults = NmapResults::parse(&contents).unwrap();

        return Ok(());
    }

    println!("Script result: {:?}", result.err());

    Ok(())
}
