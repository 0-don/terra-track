use anyhow::Ok;
use scanner::types::NmapXML;

pub async fn parse_nmap_results(data: NmapXML) -> anyhow::Result<()> {
    println!("Nmap results: {:?}", data);
    Ok(())
    
}
