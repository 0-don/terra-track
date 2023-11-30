use crate::entity::ip_host_script_e::ip_host_script_m;
use crate::entity::ip_location_e::ip_location_m;
use crate::entity::ip_main_e::ip_main_m;
use crate::entity::ip_os_e::ip_os_m;
use crate::entity::ip_service_e::ip_service_m;
use crate::entity::ip_service_script_e::ip_service_script_m;
use crate::mapper::ip_os_mapper::process_os;
use crate::mapper::ip_service_mapper::process_service;
use crate::mapper::ip_service_script_mapper::process_service_scripts;
use crate::mapper::{ip_host_script_mapper, ip_location_mapper};
use crate::printlog;
use scanner::types::{Nmap, PortUnion};

pub const BATCH_SIZE: i32 = 20;

pub async fn parse_nmap_results(nmap: &Nmap) -> anyhow::Result<()> {
    printlog!("Parsing nmap results Start");
    let host = nmap.nmaprun.host.as_ref().unwrap();
    let ip = &host.address.addr;
    let ports = &host.ports.port;

    // MAIN IP
    let ip_main = ip_main_m::Mutation::upsert_ip_main_by_ip(ip, &host.address.addrtype).await?;

    // SERVICES
    let mut services_to_create = Vec::new();
    match ports {
        PortUnion::PortArray(ports) => {
            for port in ports {
                services_to_create.push(process_service(ip_main.id, &port));
            }
        }
        PortUnion::Port(port) => services_to_create.push(process_service(ip_main.id, &port)),
    };

    let created_services =
        ip_service_m::Mutation::create_many_ip_services(services_to_create).await?;

    // HOST SCRIPTS && POST SCRIPTS
    if host.hostscript.is_some() || nmap.nmaprun.postscript.is_some() {
        let host_scripts = ip_host_script_mapper::process_host_script(
            ip_main.id,
            &host.hostscript,
            &nmap.nmaprun.postscript,
        )
        .await?;

        // IP LOCATION
        let ip_location = ip_location_mapper::parse_location(&ip_main.id, &host_scripts);
        if let Ok(ip_location) = ip_location {
            ip_location_m::Mutation::create_ip_location(ip_location).await?;
        }

        ip_host_script_m::Mutation::create_many_ip_host_script(host_scripts).await?;
    }

    // OS
    if let Some(os) = &host.os {
        let os = process_os(ip_main.id, os);
        if let Some(os) = os {
            ip_os_m::Mutation::create_ip_os(os).await?;
        }
    }

    // PORT SCRIPTS
    let mut script_models = Vec::new();
    match ports {
        PortUnion::PortArray(ports_array) => {
            for (created_service, port) in created_services.iter().zip(ports_array.iter()) {
                if let Some(script) = &port.script {
                    script_models.extend(process_service_scripts(
                        ip_main.id,
                        created_service.id,
                        script,
                    ));
                }
            }
        }
        PortUnion::Port(single_port) => {
            if let Some(created_service) = created_services.first() {
                if let Some(script) = &single_port.script {
                    script_models.extend(process_service_scripts(
                        ip_main.id,
                        created_service.id,
                        script,
                    ));
                }
            }
        }
    };
    ip_service_script_m::Mutation::create_many_ip_service_scripts(script_models).await?;

    printlog!("Parsing nmap results End");
    Ok(())
}
