mod scanner;
mod scripts;
use crate::scripts::Script;
use scanner::Scanner;
use std::net::IpAddr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let ip: IpAddr = "45.33.32.156".parse()?;
    let ports = Scanner::new(ip).run().await?;

    println!("IP {:?} Open ports: {:?}", ip.to_string(), ports);

    let script = Script::new(ip, ports);
    match script.run() {
        Ok(script_result) => println!("Script result: {}", script_result),
        Err(e) => println!("Error running script: {}", e),
    }

    Ok(())
}

