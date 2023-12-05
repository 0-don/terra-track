#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::thread::spawn(|| scanner::main());
    let _ = std::thread::spawn(|| api::main()).join();

    Ok(())
}
