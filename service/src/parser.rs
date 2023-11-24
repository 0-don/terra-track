use crate::mapper::ip_service_script_mapper::process_scripts;
use crate::models::ip_main::ip_main_m;
use crate::models::ip_service::ip_service_m;
use crate::models::ip_service_script::ip_service_script_m;
use crate::printlog;
use crate::utils::parse_os_from_nmap_output;
use entity::ip_service;
use scanner::types::Nmap;
use sea_orm::Set;

pub const BATCH_SIZE: i32 = 1;

pub async fn parse_nmap_results(nmap: &Nmap) -> anyhow::Result<()> {
    printlog!("Parsing nmap results Start");
    let host = &nmap.nmaprun.host;
    let ip = &host.address.addr;
    let ports = &host.ports.port;

    let ip_main = ip_main_m::Mutation::upsert_ip_main_by_ip(ip).await?;

    let mut services_to_create = Vec::new();
    for port in ports {
        let mut cpuarch = None;
        let mut ostype = port.service.ostype.clone();
        if let Some(servicefp) = &port.service.servicefp {
            let (os_type, cpu_arch) = parse_os_from_nmap_output(servicefp.clone());
            if os_type.is_some() && ostype.is_none() {
                ostype = os_type;
            }
            cpuarch = cpu_arch;
        }
        services_to_create.push(ip_service::ActiveModel {
            ip_main_id: Set(ip_main.id),
            protocol: Set(port.protocol.clone()),
            port: Set(port.portid as i16),
            name: Set(port.service.name.clone()),
            conf: Set(port.service.conf),
            version: Set(port.service.version.clone()),
            product: Set(port.service.product.clone()),
            extra_info: Set(port.service.extrainfo.clone()),
            tunnel: Set(port.service.tunnel.clone()),
            proto: Set(port.service.proto.clone()),
            rpcnum: Set(port.service.rpcnum.clone()),
            lowver: Set(port.service.lowver.clone()),
            highver: Set(port.service.highver.clone()),
            hostname: Set(port.service.hostname.clone()),
            method: Set(port.service.method.clone()),
            os_type: Set(ostype),
            cpu_arch: Set(cpuarch),
            device_type: Set(port.service.devicetype.clone()),
            service_fp: Set(port.service.servicefp.clone()),
            cpe: Set(port.service.cpe.clone()),
            ..Default::default()
        });
    }

    let created_services =
        ip_service_m::Mutation::create_many_ip_services(services_to_create).await?;

    let mut script_models = Vec::new();
    for (created_service, port) in created_services.iter().zip(ports.iter()) {
        if let Some(script) = &port.script {
            script_models.extend(process_scripts(ip_main.id, created_service.id, script));
        }
    }

    ip_service_script_m::Mutation::create_many_ip_service_scripts(script_models).await?;

    printlog!("Parsing nmap results End");
    Ok(())
}
