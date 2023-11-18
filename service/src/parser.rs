
use scanner::types::NmapXML;

pub async fn parse_nmap_results(data: NmapXML) -> anyhow::Result<()> {
    println!("Nmap results: {:?}", data);
    let first_host = data.host.first().unwrap();
    let ip = &first_host.address.first().unwrap().addr;
    // let ip = data.host.
    // data.scanner;
    Ok(())
}
