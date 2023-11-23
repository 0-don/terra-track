use crate::mapper::ip_service_script_mapper::process_scripts;
use crate::models::ip_main_service::ip_main_m;
use crate::models::ip_service_service::ip_service_m;
use crate::models::ip_service_service::ip_service_q;
use crate::utils::date;
use chrono::Duration;
use entity::ip_main;
use entity::ip_service;
use scanner::types::{Nmap, Port};
use sea_orm::Set;

pub const BATCH_SIZE: i32 = 1;

pub async fn parse_nmap_results(nmap: &Nmap) -> anyhow::Result<()> {
    let host = &nmap.nmaprun.host;
    let ip = &host.address.addr;
    let ports = &host.ports.port;

    let ip_main = ip_main_m::Mutation::upsert_ip_main_by_ip(ip).await?;
    for port in ports {
        process_port(&ip_main, port).await?;
    }
    Ok(())
}

async fn process_port(ip_main: &ip_main::Model, port: &Port) -> anyhow::Result<()> {
    if ip_service_q::Query::find_ip_service_by_port_and_ip_main_id_older_then(
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
    if let Some(script) = &port.script {
        process_scripts(ip_main.id, ip_service.id, &script).await?;
    }

    Ok(())
}

async fn create_ip_service(ip_main_id: i64, port: &Port) -> anyhow::Result<ip_service::Model> {
    // let (mut os_type, cpu_arch) = parse_os_from_nmap_output(&port.service.servicefp);
    // if let Some(ostype) = &port.service.ostype {
    //     os_type = Some(ostype.clone());
    // }
    ip_service_m::Mutation::create_ip_service(ip_service::ActiveModel {
        ip_main_id: Set(ip_main_id),
        port: Set(port.portid as i16),
        name: Set(port.service.name.clone()),
        product: Set(port.service.product.clone()),
        method: Set(format!("{:?}", port.service.method)),

        ..Default::default()
    })
    .await
}
