use entity::ip_main;
use scanner::types::NmapXML;
use sea_orm::Set;

use crate::models::ip_main_service;

pub async fn parse_nmap_results(data: NmapXML) -> anyhow::Result<()> {
    println!("Nmap results: {:?}", data);
    let first_host = data.host.first().unwrap();
    let ip = &first_host.address.first().unwrap().addr;
    let ports = &first_host.ports.port;

    let mut ip_main = ip_main_service::Query::find_ip_main_by_ip(ip).await?;

    if ip_main.is_none() {
        ip_main = Some(
            ip_main_service::Mutation::create_ip_main(ip_main::ActiveModel {
                ip_address: Set(ip.to_string()),
                ..Default::default()
            })
            .await?,
        );
    }


    // let ip = data.host.
    // data.scanner;
    
    
    
    Ok(())
}
