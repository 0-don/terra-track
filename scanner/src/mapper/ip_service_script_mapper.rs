use crate::{mapper::process_single_script, types::ScriptUnion};
use entity::ip_service_script;
use sea_orm::Set;

pub fn process_service_scripts(
    ip_main_id: i64,
    ip_service_id: i64,
    script_union: &ScriptUnion,
) -> Vec<ip_service_script::ActiveModel> {
    match script_union {
        ScriptUnion::Script(script) => {
            vec![ip_service_script::ActiveModel {
                ip_main_id: Set(ip_main_id),
                ip_service_id: Set(ip_service_id),
                key: Set(script.id.clone()),
                value: Set(process_single_script(script)),
                ..Default::default()
            }]
        }
        ScriptUnion::ScriptArray(scripts) => {
            let mut models = Vec::new();
            for script in scripts {
                models.push(ip_service_script::ActiveModel {
                    ip_main_id: Set(ip_main_id),
                    ip_service_id: Set(ip_service_id),
                    key: Set(script.id.clone()),
                    value: Set(process_single_script(script)),
                    ..Default::default()
                });
            }
            models
        }
    }
}
