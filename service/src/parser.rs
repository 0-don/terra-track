use crate::mapper::ip_service_script_mapper::process_scripts;
use crate::models::ip_main_service::ip_main_m;
use crate::models::ip_service_script_service::ip_service_script_m;
use crate::models::ip_service_service::ip_service_m;
use crate::printlog;
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

    // Collect all ip_service::ActiveModel instances
    let mut services_to_create = Vec::new();
    for port in ports {
        services_to_create.push(ip_service::ActiveModel {
            ip_main_id: Set(ip_main.id),
            port: Set(port.portid as i16),
            name: Set(port.service.name.clone()),
            product: Set(port.service.product.clone()),
            method: Set(format!("{:?}", port.service.method)),
            ..Default::default()
        });
    }

    // Batch create ip_service::ActiveModel instances
    let created_services =
        ip_service_m::Mutation::create_many_ip_services(services_to_create).await?;

    // Process and batch create scripts
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
