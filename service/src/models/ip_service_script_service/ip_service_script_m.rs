use crate::db::get_db_connection;
use entity::ip_service_script;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, TryIntoModel};

pub struct Mutation;

impl Mutation {
    pub async fn create_ip_service_script(
        active_model: ip_service_script::ActiveModel,
    ) -> anyhow::Result<ip_service_script::Model> {
        let db = get_db_connection().await?;
        let model = active_model.save(&db).await?.try_into_model()?;

        Ok(model)
    }

    pub async fn upsert_ip_service_script(
        ip_main_id: i64,
        ip_service_id: i64,
        key: &str,
        value: serde_json::Value,
    ) -> anyhow::Result<ip_service_script::Model> {
        let db = get_db_connection().await?;
        let mut model = ip_service_script::Entity::find()
            .filter(ip_service_script::Column::IpMainId.eq(ip_main_id))
            .filter(ip_service_script::Column::IpServiceId.eq(ip_service_id))
            .filter(ip_service_script::Column::Key.eq(key))
            .one(&db)
            .await?
            .map(|model| model.try_into_model())
            .transpose()?;

        if model.is_none() {
            model = ip_service_script::ActiveModel {
                ip_main_id: Set(ip_main_id),
                ip_service_id: Set(ip_service_id),
                key: Set(key.to_string()),
                value: Set(value.clone()),
                ..Default::default()
            }
            .insert(&db)
            .await
            .ok();
        }

        Ok(model.unwrap())
    }

    pub async fn update_ip_service_script(
        id: i64,
        model: ip_service_script::Model,
    ) -> anyhow::Result<ip_service_script::Model> {
        let db = get_db_connection().await?;
        let model = ip_service_script::ActiveModel {
            id: Set(id),
            ..model.into()
        }
        .save(&db)
        .await?
        .try_into_model()?;

        Ok(model)
    }

    pub async fn delete_ip_service_script(id: i64) -> anyhow::Result<bool> {
        let db = get_db_connection().await?;
        ip_service_script::ActiveModel {
            id: Set(id),
            ..Default::default()
        }
        .delete(&db)
        .await?;

        Ok(true)
    }

    pub async fn delete_ip_service_script_by_ip_service_id(
        ip_service_id: i64,
    ) -> anyhow::Result<bool> {
        let db = get_db_connection().await?;
        ip_service_script::Entity::delete_many()
            .filter(ip_service_script::Column::IpServiceId.eq(ip_service_id))
            .exec(&db)
            .await?;

        Ok(true)
    }
}
