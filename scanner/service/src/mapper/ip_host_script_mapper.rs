use super::process_single_script;
use entity::ip_host_script;
use parser::types::{Hostscript, ScriptUnion};
use sea_orm::Set;

pub async fn process_host_script(
    ip_main_id: i64,
    host_script: &Option<Hostscript>,
    post_script: &Option<Hostscript>,
) -> anyhow::Result<Vec<ip_host_script::ActiveModel>> {
    let mut scripts = Vec::new();

    // Process host_script using double Some pattern
    if let Some(host_script) = host_script {
        if let Some(ScriptUnion::Script(script)) = host_script.script.as_ref() {
            scripts.push(ip_host_script::ActiveModel {
                ip_main_id: Set(ip_main_id),
                value: Set(process_single_script(script)),
                key: Set(script.id.clone()),
                ..Default::default()
            });
        } else if let Some(ScriptUnion::ScriptArray(script_array)) = host_script.script.as_ref() {
            for script in script_array {
                scripts.push(ip_host_script::ActiveModel {
                    ip_main_id: Set(ip_main_id),
                    value: Set(process_single_script(script)),
                    key: Set(script.id.clone()),
                    ..Default::default()
                });
            }
        }
    }

    // Process post_script using double Some pattern
    if let Some(post_script) = post_script {
        if let Some(ScriptUnion::Script(script)) = post_script.script.as_ref() {
            scripts.push(ip_host_script::ActiveModel {
                ip_main_id: Set(ip_main_id),
                value: Set(process_single_script(script)),
                key: Set(script.id.clone()),
                ..Default::default()
            });
        } else if let Some(ScriptUnion::ScriptArray(script_array)) = post_script.script.as_ref() {
            for script in script_array {
                scripts.push(ip_host_script::ActiveModel {
                    ip_main_id: Set(ip_main_id),
                    value: Set(process_single_script(script)),
                    key: Set(script.id.clone()),
                    ..Default::default()
                });
            }
        }
    }

    Ok(scripts)
}
