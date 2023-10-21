mod scanner;
mod scripts;
use scanner::Scanner;
mod port_strategy;
use crate::scripts::Script;
use futures::executor::block_on;
use port_strategy::PortStrategy;
use std::net::IpAddr;

fn main() {
    // read file as string ./nmap.xml

    // let mut file = File::open("./nmap.xml").unwrap();
    // let mut contents = String::new();
    // file.read_to_string(&mut contents).unwrap();
    // let _results = NmapResults::parse(&contents).unwrap();

    let ips: IpAddr = "45.33.32.156".parse().unwrap();
    let scanner = Scanner::new(ips);
    let ports_per_ip = block_on(scanner.run());

    for (ip, ports) in &ports_per_ip {
        let script = Script::build(*ip, ports.clone());
        match script.run() {
            Ok(script_result) => println!("Script result: {}", script_result),
            Err(e) => println!("Error running script: {}", e),
        }
    }
}
