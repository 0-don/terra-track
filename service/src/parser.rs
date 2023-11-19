use std::collections::HashMap;

use chrono::Duration;
use entity::ip_main;
use regex::Regex;
use scanner::types::NmapXML;
use sea_orm::Set;

use crate::models::{ip_main_service, ip_service_service};

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
        let days_ago_30 = chrono::Utc::now()
            .with_timezone(&chrono::FixedOffset::east_opt(0).unwrap())
            - Duration::days(30);
        let ip_service =
            ip_service_service::Query::find_ip_service_by_port_and_ip_main_id_older_then(
                port.portid.parse::<i16>().unwrap(),
                ip_main.as_ref().unwrap().id,
                Some(days_ago_30),
            )
            .await?;

        let (os_type, cpu_arch) = parse_os_from_nmap_output(&port.service.servicefp);
        println!("OS Type: {:?}, CPU Arch: {:?}", os_type, cpu_arch);

        // if ip_service.is_none() {
        //     ip_service = Some(
        //         ip_service_service::Mutation::create_ip_service(ip_main_service::ActiveModel {
        //             ip_main_id: Set(ip_main.as_ref().unwrap().id),
        //             protocol: Set(port.protocol.to_string()),
        //             port: Set(port.portid),
        //             name: Set(port.service.name.to_string()),
        //             ..Default::default()
        //         })
        //         .await?,
        //     );
        // }
    }

    // let ip = data.host.
    // data.scanner;

    Ok(())
}

pub fn parse_os_from_nmap_output(nmap_output: &Option<String>) -> (Option<String>, Option<String>) {
    if nmap_output.is_none() {
        return (None, None);
    }
    let os_patterns = vec![
        r"windows\s\d+-x86_64|linux-gnueabihf-armv\d+", // Specific OS versions with architecture
        r"linux|ubuntu|debian|centos|fedora|windows\s\d+|mac\sos|solaris|bsd|sunos|gnu", // OS Types including distributions
    ];
    let cpu_patterns = vec![
        r"(x86_64|x86|aarch64|armv\d+|mips\d+|sparc|ppc64|s390x)-pc-(linux-gnu|windows-gnu|solaris|bsd)", // Combined OS and Architecture
        r"x86_64|x86|aarch64|armv\d+|mips\d+|sparc|ppc64|s390x", // CPU Architectures
    ];

    let mut os_counts = HashMap::new();
    let mut cpu_counts = HashMap::new();

    // Process OS patterns
    for pattern in os_patterns {
        let re = Regex::new(pattern).unwrap();
        for line in nmap_output.as_ref().unwrap().lines() {
            if let Some(cap) = re.captures(line) {
                *os_counts.entry(cap[0].to_string()).or_insert(0) += 1;
            }
        }
    }

    // Process CPU patterns
    for pattern in cpu_patterns {
        let re = Regex::new(pattern).unwrap();
        for line in nmap_output.as_ref().unwrap().lines() {
            if let Some(cap) = re.captures(line) {
                *cpu_counts.entry(cap[0].to_string()).or_insert(0) += 1;
            }
        }
    }

    let os_type = os_counts
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(match_str, _)| match_str);
    let cpu_arch = cpu_counts
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(match_str, _)| match_str);

    (os_type, cpu_arch)
}
