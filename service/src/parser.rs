use crate::entity_mapper::process_scripts;
use crate::models::{ip_main_service, ip_service_service};
use crate::utils::{date, parse_os_from_nmap_output};
use chrono::Duration;
use entity::ip_main;
use entity::ip_service;
use scanner::types::{Nmap, Port};
use sea_orm::Set;

pub async fn parse_nmap_results(nmap: &Nmap) -> anyhow::Result<()> {
    let host = &nmap.nmaprun.host;
    let ip = &host.address.addr;
    let ports = &host.ports.port;

    let ip_main = ip_main_service::Mutation::upsert_ip_main_by_ip(ip).await?;
    for port in ports {
        process_port(&ip_main, port).await?;
    }
    Ok(())
}

async fn process_port(ip_main: &ip_main::Model, port: &Port) -> anyhow::Result<()> {
    if ip_service_service::Query::find_ip_service_by_port_and_ip_main_id_older_then(
        port.portid as i16,
        ip_main.id,
        Some(date(Duration::days(365))),
    )
    .await?
    .is_some()
    {
        return Ok(());
    }
    let ip_service = create_ip_service(ip_main.id, port).await?;
    process_scripts(ip_main.id, ip_service.id, &port.script).await
}

async fn create_ip_service(ip_main_id: i64, port: &Port) -> anyhow::Result<ip_service::Model> {
    let (mut os_type, cpu_arch) = parse_os_from_nmap_output(&port.service.servicefp);
    if let Some(ostype) = &port.service.ostype {
        os_type = Some(ostype.clone());
    }
    ip_service_service::Mutation::create_ip_service(ip_service::ActiveModel {
        ip_main_id: Set(ip_main_id),
        port: Set(port.portid as i16),
        name: Set(port.service.name.clone()),
        product: Set(port.service.product.clone()),
        service_fp: Set(port.service.servicefp.clone()),
        version: Set(port.service.version.clone()),
        extra_info: Set(port.service.extrainfo.clone()),
        method: Set(format!("{:?}", port.service.method)),
        os_type: Set(os_type),
        cpu_arch: Set(cpu_arch),
        ..Default::default()
    })
    .await
}
