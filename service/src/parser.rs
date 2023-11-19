use entity::ip_main;
use regex::Regex;
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

    for port in ports {
        let mut ip_service = ip_main_service::Query::find_ip_service_by_ip_main_id_older_then(
            ip_main.as_ref().unwrap().id,
            port.portid,
        )
        .await?;

        if ip_service.is_none() {
            ip_service = Some(
                ip_main_service::Mutation::create_ip_service(ip_main_service::ActiveModel {
                    ip_main_id: Set(ip_main.as_ref().unwrap().id),
                    protocol: Set(port.protocol.to_string()),
                    port: Set(port.portid),
                    name: Set(port.service.name.to_string()),
                    ..Default::default()
                })
                .await?,
            );
        }
    }




    // let ip = data.host.
    // data.scanner;
    
    
    
    Ok(())
}


pub fn parse_os_from_nmap_output(nmap_output: &str) -> Vec<String> {
    let patterns = vec![
        r"x86_64|aarch64|arm|sparc|mips", // CPU Architectures
        r"linux|windows|gnu|sunos|bsd",   // OS Vendors or Types
        // Add more patterns here as needed
    ];

    let mut os_info = Vec::new();
    for pattern in patterns {
        let re = Regex::new(pattern).unwrap();
        for line in nmap_output.lines() {
            for cap in re.captures_iter(line) {
                os_info.push(cap[0].to_string());
            }
        }
    }

    os_info
}