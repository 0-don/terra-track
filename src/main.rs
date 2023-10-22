mod scanner;
mod scripts;
use crate::scripts::Script;
use scanner::Scanner;
use std::net::IpAddr;

#[tokio::main]
async fn main() {
    let ip: IpAddr = "45.33.32.156".parse().unwrap();
    let ports = Scanner::new(ip).run().await;
    
    let script = Script::build(ip, ports);
    match script.run() {
        Ok(script_result) => println!("Script result: {}", script_result),
        Err(e) => println!("Error running script: {}", e),
    }
}

// read file as string ./nmap.xml

// let mut file = File::open("./nmap.xml").unwrap();
// let mut contents = String::new();
// file.read_to_string(&mut contents).unwrap();
// let _results = NmapResults::parse(&contents).unwrap();
