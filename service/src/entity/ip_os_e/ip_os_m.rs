use crate::db::get_db_connection;
use ::entity::ip_os;
use sea_orm::{ActiveModelTrait, Set, TryIntoModel};

pub struct Mutation;

impl Mutation {
    pub async fn create_ip_os(active_model: ip_os::ActiveModel) -> anyhow::Result<ip_os::Model> {
        let db = get_db_connection().await?;
        let model = active_model.save(&db).await?.try_into_model()?;

        Ok(model)
    }

    pub async fn update_ip_os(id: i64, model: ip_os::Model) -> anyhow::Result<ip_os::Model> {
        let db = get_db_connection().await?;
        let model = ip_os::ActiveModel {
            id: Set(id),
            ..model.into()
        }
        .save(&db)
        .await?
        .try_into_model()?;

        Ok(model)
    }

    pub async fn delete_ip_os(id: i64) -> anyhow::Result<bool> {
        let db = get_db_connection().await?;
        ip_os::ActiveModel {
            id: Set(id),
            ..Default::default()
        }
        .delete(&db)
        .await?;

        Ok(true)
    }
}
