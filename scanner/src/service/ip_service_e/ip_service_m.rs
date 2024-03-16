use crate::utils::date;
use ::entity::ip_service;
use chrono::Duration;
use migration::db::get_db_connection;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, TryIntoModel};

pub struct Mutation;

impl Mutation {
    pub async fn create_ip_service(
        active_model: ip_service::ActiveModel,
    ) -> anyhow::Result<ip_service::Model> {
        let db = get_db_connection().await?;
        let model = active_model.save(&db).await?.try_into_model()?;

        Ok(model)
    }

    pub async fn update_ip_service(
        id: i64,
        model: ip_service::Model,
    ) -> anyhow::Result<ip_service::Model> {
        let db = get_db_connection().await?;
        let model = ip_service::ActiveModel {
            id: Set(id),
            ..model.into()
        }
        .save(&db)
        .await?
        .try_into_model()?;

        Ok(model)
    }

    pub async fn delete_ip_service(id: i64) -> anyhow::Result<bool> {
        let db = get_db_connection().await?;
        ip_service::ActiveModel {
            id: Set(id),
            ..Default::default()
        }
        .delete(&db)
        .await?;

        Ok(true)
    }

    pub async fn create_many_ip_services(
        active_models: Vec<ip_service::ActiveModel>,
    ) -> anyhow::Result<Vec<ip_service::Model>> {
        if active_models.is_empty() {
            return Ok(vec![]);
        }

        let db = get_db_connection().await?;
        let ip_main_id = active_models[0].ip_main_id.clone(); // Assuming ip_main_id is present
        ip_service::Entity::insert_many(active_models)
            .on_empty_do_nothing()
            .exec(&db)
            .await?;

        // Fetching records created in the last 5 minutes
        let five_minutes_ago = date(Duration::try_minutes(5).unwrap());
        let inserted_models = ip_service::Entity::find()
            .filter(ip_service::Column::IpMainId.eq(ip_main_id.unwrap())) // Adjust based on your field's actual name
            .filter(ip_service::Column::CreatedAt.gt(five_minutes_ago))
            .all(&db)
            .await?;

        Ok(inserted_models)
    }
}
