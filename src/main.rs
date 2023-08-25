use std::process::Command;
use std::str;

#[tokio::main]
async fn main() {
    let mut command = Command::new("nmap");
    let output = command.output().unwrap();

    // stdout to utf8
    println!(
        "{:?}",
        str::from_utf8(output.stdout.as_slice()).unwrap()
    );

    println!("Hello, world!");
}
