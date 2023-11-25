use entity::ip_os;
use scanner::types::Os;
use sea_orm::Set;

pub async fn process_os(ip_main_id: i64, os: &Os) -> anyhow::Result<ip_os::ActiveModel> {
    Ok(ip_os::ActiveModel {
        ip_main_id: Set(ip_main_id),
        ..Default::default()
    })
}
