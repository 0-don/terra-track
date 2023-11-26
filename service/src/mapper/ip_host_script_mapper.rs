use super::process_single_script;
use entity::ip_host_script;
use scanner::types::ScriptUnion;
use sea_orm::Set;

pub async fn process_host_script(
    ip_main_id: i64,
    host_script: &Option<ScriptUnion>,
    post_script: &Option<ScriptUnion>,
) -> anyhow::Result<Vec<ip_host_script::ActiveModel>> {
    let mut scripts = Vec::new();

    // Process host_script
    if let Some(host_script) = host_script {
        match host_script {
            ScriptUnion::Script(script) => {
                scripts.push(ip_host_script::ActiveModel {
                    ip_main_id: Set(ip_main_id),
                    value: Set(process_single_script(script)),
                    key: Set(script.id.clone()),
                    ..Default::default()
                });
            }
            ScriptUnion::ScriptArray(script_array) => {
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
    }

    // Process post_script
    if let Some(post_script) = post_script {
        match post_script {
            ScriptUnion::Script(script) => {
                scripts.push(ip_host_script::ActiveModel {
                    ip_main_id: Set(ip_main_id),
                    value: Set(process_single_script(script)),
                    key: Set(script.id.clone()),
                    ..Default::default()
                });
            }
            ScriptUnion::ScriptArray(script_array) => {
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
    }

    Ok(scripts)
}
