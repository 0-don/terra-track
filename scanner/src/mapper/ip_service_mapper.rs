use super::ip_os_mapper::parse_os_from_nmap_output;
use crate::types::{CpeUnion, Service};
use entity::ip_service;
use sea_orm::Set;
use serde_json::json;

pub fn process_service(
    ip_main_id: i64,
    service: &Service,
    protocol: &String,
    portid: &i32,
) -> ip_service::ActiveModel {
    let mut cpuarch = None;
    let mut ostype = service.ostype.clone();
    if let Some(servicefp) = &service.servicefp {
        let (os_type, cpu_arch) = parse_os_from_nmap_output(servicefp.clone());
        if os_type.is_some() && ostype.is_none() {
            ostype = os_type;
        }
        cpuarch = cpu_arch;
    }

    let mut cpe: Option<String> = None;

    if service.cpe.is_some() {
        cpe = match &service.cpe.clone().unwrap() {
            CpeUnion::CpeArray(cpe_array) => Some(cpe_array.join(",")),
            CpeUnion::Cpe(cpe) => Some(cpe.clone()),
        };
    }

    let version = match service.version.clone() {
        Some(version) => Some(json!(version).to_string().replace("\"", "")),
        None => None,
    };

    return ip_service::ActiveModel {
        ip_main_id: Set(ip_main_id),
        protocol: Set(protocol.clone()),
        port: Set(portid.clone()),
        name: Set(service.name.clone()),
        conf: Set(service.conf as i16),
        version: Set(version),
        product: Set(service.product.clone()),
        extra_info: Set(service.extrainfo.clone()),
        tunnel: Set(service.tunnel.clone()),
        proto: Set(service.proto.clone()),
        rpcnum: Set(service.rpcnum.clone()),
        lowver: Set(service.lowver.clone()),
        highver: Set(service.highver.clone()),
        hostname: Set(service.hostname.clone()),
        method: Set(service.method.clone()),
        os_type: Set(ostype),
        cpu_arch: Set(cpuarch),
        device_type: Set(service.devicetype.clone()),
        service_fp: Set(service.servicefp.clone()),
        cpe: Set(cpe),
        ..Default::default()
    };
}
