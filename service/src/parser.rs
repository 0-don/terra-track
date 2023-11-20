use crate::models::{ip_main_service, ip_service_extra_service, ip_service_service};
use crate::utils::date;
use chrono::Duration;
use entity::ip_main;
use entity::ip_service;
use regex::Regex;
use scanner::types::{
    ElemUnion, Nmap, Port, PurpleScript, PurpleTable, ScriptElement, ScriptTable, ScriptUnion,
    TableTableUnion,
};
use sea_orm::Set;
use serde_json::json;
use std::collections::HashMap;

pub async fn parse_nmap_results(nmap: Nmap) -> anyhow::Result<()> {
    let host = nmap.nmaprun.host;
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
        method: Set(format!("{:?}", port.service.method.clone())),
        os_type: Set(os_type),
        cpu_arch: Set(cpu_arch),
        ..Default::default()
    })
    .await
}

async fn process_scripts(
    ip_main_id: i64,
    ip_service_id: i64,
    script_union: &ScriptUnion,
) -> anyhow::Result<()> {
    match script_union {
        ScriptUnion::PurpleScript(purple_script) => {
            process_purple_script(ip_main_id, ip_service_id, purple_script).await
        }
        ScriptUnion::ScriptElementArray(script_elements) => {
            process_script_elements(ip_main_id, ip_service_id, script_elements).await
        }
    }
}

async fn process_purple_script(
    ip_main_id: i64,
    ip_service_id: i64,
    purple_script: &PurpleScript,
) -> anyhow::Result<()> {
    let data = purple_script
        .elem
        .iter()
        .map(|elem| (elem.key.clone(), elem.value.clone()))
        .collect::<HashMap<_, _>>();

    ip_service_extra_service::Mutation::upsert_ip_service_extra(
        ip_main_id,
        ip_service_id,
        &purple_script.id,
        json!(data),
    )
    .await?;

    Ok(())
}

async fn process_script_elements(
    ip_main_id: i64,
    ip_service_id: i64,
    script_elements: &[ScriptElement],
) -> anyhow::Result<()> {
    for script in script_elements {
        if let Some(value) = construct_script_element_value(script) {
            ip_service_extra_service::Mutation::upsert_ip_service_extra(
                ip_main_id,
                ip_service_id,
                &script.id,
                value,
            )
            .await?;
        }
    }

    Ok(())
}

fn construct_script_element_value(script: &ScriptElement) -> Option<serde_json::Value> {
    script
        .table
        .as_ref()
        .map(|table| json!({ "table": process_script_table(table) }))
        .or_else(|| {
            script
                .elem
                .as_ref()
                .map(|elem| json!({ "elem": process_elem_union(elem) }))
        })
}

fn process_script_table(table: &ScriptTable) -> serde_json::Value {
    match table {
        ScriptTable::IndigoTable(elem) => json!({ elem.key.as_str(): elem.elem }),
        ScriptTable::PurpleTableArray(elem_array) => {
            json!(elem_array
                .iter()
                .map(|elem| {
                    let key = elem.key.clone();
                    let value = process_table_elem(elem);
                    (key, value)
                })
                .collect::<HashMap<_, _>>())
        }
    }
}

fn process_table_elem(elem: &PurpleTable) -> serde_json::Value {
    if let Some(elems) = &elem.elem {
        json!(elems
            .iter()
            .map(|e| (e.key.clone(), e.value.clone()))
            .collect::<HashMap<_, _>>())
    } else if let Some(table) = &elem.table {
        process_table_table_union(table)
    } else {
        json!(HashMap::<String, serde_json::Value>::new())
    }
}

fn process_table_table_union(table_union: &TableTableUnion) -> serde_json::Value {
    match table_union {
        TableTableUnion::FluffyTableArray(fluffy_tables) => {
            json!(fluffy_tables
                .iter()
                .flat_map(|fluffy_table| {
                    fluffy_table
                        .elem
                        .iter()
                        .map(|e| (e.key.clone(), e.value.clone()))
                })
                .collect::<HashMap<_, _>>())
        }
        TableTableUnion::TentacledTable(tentacled_table) => {
            json!(tentacled_table
                .table
                .elem
                .iter()
                .map(|e| (e.key.clone(), e.value.clone()))
                .collect::<HashMap<_, _>>())
        }
    }
}

fn process_elem_union(elem_union: &ElemUnion) -> serde_json::Value {
    match elem_union {
        ElemUnion::ElemElem(e) => json!({ e.key.as_str(): e.value }),
        ElemUnion::ElemElemArray(elem_array) => json!(elem_array
            .iter()
            .map(|elem| { (elem.key.clone(), elem.value.clone()) })
            .collect::<HashMap<_, _>>()),
        ElemUnion::String(string) => json!(string),
    }
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
