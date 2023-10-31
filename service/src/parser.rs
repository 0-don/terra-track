use scanner::types::NmapXML;

pub async fn parse_nmap_results(data: NmapXML) -> anyhow::Result<()> {
    println!("Nmap results: {:?}", data);
    data.scanner;
    Ok(())
}
