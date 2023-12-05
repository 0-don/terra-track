use migration::db::get_db_connection;
use ::entity::ip_location;
use sea_orm::{ActiveModelTrait, Set, TryIntoModel};

pub struct Mutation;

impl Mutation {
    pub async fn create_ip_location(
        active_model: ip_location::ActiveModel,
    ) -> anyhow::Result<ip_location::Model> {
        let db = get_db_connection().await?;
        let model = active_model.save(&db).await?.try_into_model()?;

        Ok(model)
    }

    pub async fn update_ip_location(
        id: i64,
        model: ip_location::Model,
    ) -> anyhow::Result<ip_location::Model> {
        let db = get_db_connection().await?;
        let model = ip_location::ActiveModel {
            id: Set(id),
            ..model.into()
        }
        .save(&db)
        .await?
        .try_into_model()?;

        Ok(model)
    }

    pub async fn delete_ip_location(id: i64) -> anyhow::Result<bool> {
        let db = get_db_connection().await?;
        ip_location::ActiveModel {
            id: Set(id),
            ..Default::default()
        }
        .delete(&db)
        .await?;

        Ok(true)
    }
}
