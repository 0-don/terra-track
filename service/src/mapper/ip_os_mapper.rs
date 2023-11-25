use crate::utils::parse_os_from_nmap_output;
use entity::ip_os;
use scanner::types::{Os, OsMatchClassUnion, Osmatch, OsmatchUnion};
use sea_orm::Set;
use serde_json::json;

pub fn process_os(ip_main_id: i64, os: &Os) -> Option<ip_os::ActiveModel> {
    if os.osmatch.is_none() {
        return None;
    }

    Some(match os.osmatch.as_ref().unwrap() {
        OsmatchUnion::OsmatchElementArray(osmatch) => {
            parse_osmatch(ip_main_id, os, &osmatch.first().unwrap())
        }
        OsmatchUnion::Osmatch(osmatch) => parse_osmatch(ip_main_id, os, &osmatch),
    })
}

pub fn parse_osmatch(ip_main_id: i64, os: &Os, osmatch: &Osmatch) -> ip_os::ActiveModel {
    let (_, cpu_arch) = parse_os_from_nmap_output(os.osfingerprint.fingerprint.clone());

    let os_class_element = match &osmatch.osclass {
        OsMatchClassUnion::OsclassElementArray(osclass) => osclass.first().unwrap(),
        OsMatchClassUnion::OsclassElement(osclass) => &osclass,
    };

    ip_os::ActiveModel {
        ip_main_id: Set(ip_main_id),
        name: Set(osmatch.name.clone()),
        fingerprint: Set(os.osfingerprint.fingerprint.clone()),
        osfamily: Set(os_class_element.osfamily.clone()),
        r#type: Set(json!(os_class_element.osclass_type.clone()).to_string()),
        vendor: Set(os_class_element.vendor.clone()),
        os_gen: Set(os_class_element.osgen.clone()),
        cpu_arch: Set(cpu_arch),
        cpe: Set(os_class_element.cpe.clone()),
        ..Default::default()
    }
}
