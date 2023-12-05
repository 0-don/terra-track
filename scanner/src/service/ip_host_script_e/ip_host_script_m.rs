use ::entity::ip_host_script;
use migration::db::get_db_connection;
use sea_orm::{ActiveModelTrait, EntityTrait, Set, TryIntoModel};

pub struct Mutation;

impl Mutation {
    pub async fn create_ip_host_script(
        active_model: ip_host_script::ActiveModel,
    ) -> anyhow::Result<ip_host_script::Model> {
        let db = get_db_connection().await?;
        let model = active_model.save(&db).await?.try_into_model()?;

        Ok(model)
    }

    pub async fn update_ip_host_script(
        id: i64,
        model: ip_host_script::Model,
    ) -> anyhow::Result<ip_host_script::Model> {
        let db = get_db_connection().await?;
        let model = ip_host_script::ActiveModel {
            id: Set(id),
            ..model.into()
        }
        .save(&db)
        .await?
        .try_into_model()?;

        Ok(model)
    }

    pub async fn delete_ip_host_script(id: i64) -> anyhow::Result<bool> {
        let db = get_db_connection().await?;
        ip_host_script::ActiveModel {
            id: Set(id),
            ..Default::default()
        }
        .delete(&db)
        .await?;

        Ok(true)
    }

    pub async fn create_many_ip_host_script(
        active_models: Vec<ip_host_script::ActiveModel>,
    ) -> anyhow::Result<bool> {
        let db = get_db_connection().await?;
        ip_host_script::Entity::insert_many(active_models)
            .exec(&db)
            .await?;

        Ok(true)
    }
}
