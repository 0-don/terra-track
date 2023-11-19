use crate::models::{ip_main_service, ip_service_extra_service, ip_service_service};
use crate::utils::date;
use chrono::Duration;
use entity::ip_service;
use entity::{ip_main, ip_service_extra};
use regex::Regex;
use scanner::types::NmapXML;
use sea_orm::Set;
use serde_json::json;
use std::collections::HashMap;

pub async fn parse_nmap_results(data: NmapXML) -> anyhow::Result<()> {
    let first_host = data.host.first().unwrap();
    let ip = &first_host.address.first().unwrap().addr;
    let ports = &first_host.ports.port;

    let ip_main = ip_main_service::Mutation::upsert_ip_main(ip_main::ActiveModel {
        ip_address: Set(ip.to_string()),
        ..Default::default()
    })
    .await?;

    for port in ports {
        let ip_service =
            ip_service_service::Query::find_ip_service_by_port_and_ip_main_id_older_then(
                port.portid.parse::<i16>().unwrap(),
                ip_main.id,
                Some(date(Duration::days(365))),
            )
            .await?;

        if ip_service.is_some() {
            continue;
        }

        let (mut os_type, cpu_arch) = parse_os_from_nmap_output(&port.service.servicefp);

        if port.service.ostype.is_some() {
            os_type = port.service.ostype.clone();
        }

        let ip_service = ip_service_service::Mutation::create_ip_service(ip_service::ActiveModel {
            ip_main_id: Set(ip_main.id),
            port: Set(port.portid.parse::<i16>().unwrap()),
            name: Set(port.service.name.clone()),
            product: Set(port.service.product.clone()),
            service_fp: Set(port.service.servicefp.clone()),
            version: Set(port.service.version.clone()),
            extra_info: Set(port.service.extrainfo.clone()),
            method: Set(port.service.method.clone()),
            os_type: Set(os_type),
            cpu_arch: Set(cpu_arch),
            ..Default::default()
        })
        .await?;

        ip_service_extra_service::Mutation::delete_ip_service_extra_by_ip_service_id(ip_service.id)
            .await?;
        if let Some(scripts) = &port.script {
            for script in scripts {
                // ip_service_extra_service::Mutation::create_ip_service_extra(
                //     ip_service_extra::ActiveModel {
                //         ip_main_id: Set(ip_main.id),
                //         ip_service_id: Set(ip_service.id),
                //         key: Set(script.id.clone()),
                //         value: Set(json!(&script.elems)),
                //         ..Default::default()
                //     },
                // )
                // .await?;

                // if !&script.elems.is_empty() {
                //     ip_service_extra_service::Mutation::create_ip_service_extra(
                //         ip_service_extra::ActiveModel {
                //             ip_main_id: Set(ip_main.id),
                //             ip_service_id: Set(ip_service.id),
                //             key: Set(script.id.clone()),
                //             value: Set(json!(&script.elems)),
                //             ..Default::default()
                //         },
                //     )
                //     .await?;
                // }

                // for table in &script.tables {
                //     if !&table.elems.is_empty() {
                //         ip_service_extra_service::Mutation::create_ip_service_extra(
                //             ip_service_extra::ActiveModel {
                //                 ip_main_id: Set(ip_main.id),
                //                 ip_service_id: Set(ip_service.id),
                //                 key: Set(table.key.as_ref().unwrap().to_owned()),
                //                 value: Set(json!(&table.elems)),
                //                 ..Default::default()
                //             },
                //         )
                //         .await?;
                //     }
                // }
            }
        }
    }

    Ok(())
}

pub fn parse_os_from_nmap_output(nmap_output: &Option<String>) -> (Option<String>, Option<String>) {
    if nmap_output.is_none() {
        return (None, None);
    }
    let os_patterns = vec![
        // Specific OS versions with architecture
        r"windows\s(server\s)?(11|10|8\.1|8|7|xp)|windows\s\d+-x86_64|linux-gnueabihf-armv\d+",
        // Expanded OS Types including distributions and variations
        r"linux|ubuntu|debian|centos|fedora|red\s?hat|suse|arch\s?linux|manjaro|mint|aix|hp-ux|solaris|bsd|sunos|gnu|vmware|xen|kvm|mac\sos\sx|macos\s(catalina|big\s?sur|monterey|sierra|high\s?sierra|mojave)|android|ios|windows\sphone",
    ];

    let cpu_patterns = vec![
        // Expanded CPU Architectures including specific Intel and AMD architectures
        r"x86_64|x86|i[3579]|ryzen|aarch64|armv\d+|mips\d+|sparc|ppc64|s390x|itanium|alpha|pa-risc|avr|pic|msp430",
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
