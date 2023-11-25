use super::process_single_script;
use entity::ip_host_script;
use scanner::types::Hostscript;
use sea_orm::Set;

pub async fn process_host_script(
    ip_main_id: i64,
    host_script: &Hostscript,
) -> anyhow::Result<Vec<ip_host_script::ActiveModel>> {
    let mut scripts = Vec::new();

    for script in &host_script.script {
        scripts.push(ip_host_script::ActiveModel {
            ip_main_id: Set(ip_main_id),
            value: Set(process_single_script(&script)),
            key: Set(script.id.clone()),
            ..Default::default()
        });
    }

    Ok(scripts)
}
