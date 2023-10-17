mod config;
use config::Opts;
mod scanner;
mod scripts;
use scanner::Scanner;
use utils::dns_resolver::parse_addresses;
mod port_strategy;
use crate::scripts::Script;
use futures::executor::block_on;
use port_strategy::PortStrategy;
use std::net::IpAddr;

mod utils;

fn main() {
    // read file as string ./nmap.xml

    // let mut file = File::open("./nmap.xml").unwrap();
    // let mut contents = String::new();
    // file.read_to_string(&mut contents).unwrap();
    // let _results = NmapResults::parse(&contents).unwrap();

    let opts: Opts = Opts {
        addresses: vec!["scanme.nmap.org".into()],
        batch_size: 4500,
        timeout: 100,
        tries: 1,
    };
    let ips: Vec<IpAddr> = parse_addresses(&opts);

    let scanner = Scanner::new(&ips);

    let ports_per_ip = block_on(scanner.run());

    for (ip, ports) in &ports_per_ip {
        let script = Script::build(*ip, ports.clone());
        match script.run() {
            Ok(script_result) => {
                println!("Script result: {}", script_result);
            }
            Err(e) => {
                println!("Error running script: {}", e);
            }
        }
    }
}
