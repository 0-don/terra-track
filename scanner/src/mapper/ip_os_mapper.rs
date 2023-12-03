use entity::ip_os;
use regex::Regex;
use sea_orm::Set;
use std::collections::HashMap;

use crate::types::{OsmatchUnion, Os, Osmatch, OsMatchClassUnion, CpeUnion};

pub fn process_os(ip_main_id: i64, os: &Os) -> Option<ip_os::ActiveModel> {
    os.osmatch.as_ref().map(|osmatch| match osmatch {
        OsmatchUnion::OsmatchElementArray(osmatch_array) => {
            parse_osmatch(ip_main_id, os, osmatch_array)
        }
        OsmatchUnion::Osmatch(osmatch) => parse_osmatch(ip_main_id, os, &[osmatch.clone()]),
    })
}

pub fn parse_osmatch(ip_main_id: i64, os: &Os, osmatch_array: &[Osmatch]) -> ip_os::ActiveModel {
    let (_, cpu_arch) = parse_os_from_nmap_output(os.osfingerprint.fingerprint.clone());

    let mut os_class_map = HashMap::new();
    for osmatch in osmatch_array {
        let osclass_elements = match &osmatch.osclass {
            OsMatchClassUnion::OsclassElementArray(array) => array,
            OsMatchClassUnion::OsclassElement(element) => std::slice::from_ref(element),
        };

        for os_class in osclass_elements {
            let key = (
                os_class.osfamily.clone(),
                os_class.vendor.clone(),
                os_class.osgen.clone(),
            );
            os_class_map
                .entry(key)
                .or_insert_with(Vec::new)
                .push(os_class);
        }
    }

    // Use references to iterate over the map
    let best_key = os_class_map
        .iter()
        .max_by_key(|(_, v)| v.len())
        .map(|(k, _)| k)
        .expect("No OS class found")
        .clone(); // Clone the key here

    let best_os_class = os_class_map.get(&best_key).unwrap().first().unwrap();
    let cpe = match &best_os_class.cpe {
        Some(cpe) => match cpe {
            CpeUnion::CpeArray(cpe) => Some(cpe.join(",")),
            CpeUnion::Cpe(cpe) => Some(cpe.clone()),
        },
        None => None,
    };

    ip_os::ActiveModel {
        ip_main_id: Set(ip_main_id),
        name: Set(osmatch_array.first().unwrap().name.clone()),
        fingerprint: Set(os.osfingerprint.fingerprint.clone()),
        osfamily: Set(best_os_class.osfamily.clone()),
        r#type: Set(best_os_class.osclass_type.clone()),
        vendor: Set(best_os_class.vendor.clone()),
        os_gen: Set(best_os_class.osgen.clone()),
        cpu_arch: Set(cpu_arch),
        cpe: Set(cpe),
        ..Default::default()
    }
}

pub fn parse_os_from_nmap_output(nmap_output: String) -> (Option<String>, Option<String>) {
    let os_patterns = vec![
        r"windows\s(server\s)?(11|10|8\.1|8|7|xp)|windows\s\d+-x86_64|linux-gnueabihf-armv\d+",
        r"linux|ubuntu|debian|centos|fedora|red\s?hat|suse|arch\s?linux|manjaro|mint|aix|hp-ux|solaris|bsd|sunos|gnu|vmware|xen|kvm|mac\sos\sx|macos\s(catalina|big\s?sur|monterey|sierra|high\s?sierra|mojave)|android|ios|windows\sphone",
    ];
    let cpu_patterns = vec![
        r"x86_64|x86|i[3579]|ryzen|aarch64|armv\d+|mips\d+|sparc|ppc64|s390x|itanium|alpha|pa-risc|avr|pic|msp430",
    ];
    let mut os_counts = HashMap::new();
    let mut cpu_counts = HashMap::new();
    for pattern in os_patterns {
        let re = Regex::new(pattern).unwrap();
        for line in nmap_output.lines() {
            if let Some(cap) = re.captures(line) {
                *os_counts.entry(cap[0].to_string()).or_insert(0) += 1;
            }
        }
    }
    for pattern in cpu_patterns {
        let re = Regex::new(pattern).unwrap();
        for line in nmap_output.lines() {
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
