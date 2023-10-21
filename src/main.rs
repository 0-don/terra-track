mod scanner;
mod scripts;
use scanner::Scanner;
use crate::scripts::Script;
use futures::executor::block_on;
use std::net::IpAddr;

fn main() {
    // read file as string ./nmap.xml

    // let mut file = File::open("./nmap.xml").unwrap();
    // let mut contents = String::new();
    // file.read_to_string(&mut contents).unwrap();
    // let _results = NmapResults::parse(&contents).unwrap();

    let ip: IpAddr = "45.33.32.156".parse().unwrap();
    let ports_per_ip = block_on(Scanner::new(ip).run());

    for (ip, ports) in &ports_per_ip {
        let script = Script::build(*ip, ports.clone());
        match script.run() {
            Ok(script_result) => println!("Script result: {}", script_result),
            Err(e) => println!("Error running script: {}", e),
        }
    }
}
