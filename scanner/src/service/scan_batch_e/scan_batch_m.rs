use ::entity::scan_batch;
use migration::db::get_db_connection;
use sea_orm::{ActiveModelTrait, EntityTrait, QueryOrder, Set, TryIntoModel};

pub struct Mutation;

impl Mutation {
    pub async fn create_scan_batch(
        active_model: scan_batch::ActiveModel,
    ) -> anyhow::Result<scan_batch::Model> {
        let db = get_db_connection().await?;
        let model = active_model.save(&db).await?.try_into_model()?;

        Ok(model)
    }

    pub async fn update_scan_batch(
        model: scan_batch::ActiveModel,
    ) -> anyhow::Result<scan_batch::Model> {
        let db = get_db_connection().await?;
        let model = scan_batch::ActiveModel {
            id: model.id.clone(),
            ..model.into()
        }
        .save(&db)
        .await?
        .try_into_model()?;

        Ok(model)
    }

    pub async fn delete_scan_batch(id: i64) -> anyhow::Result<bool> {
        let db = get_db_connection().await?;
        scan_batch::ActiveModel {
            id: Set(id),
            ..Default::default()
        }
        .delete(&db)
        .await?;

        Ok(true)
    }

    pub async fn delete_all_scan_batch() -> anyhow::Result<bool> {
        let db = get_db_connection().await?;
        scan_batch::Entity::delete_many().exec(&db).await?;

        Ok(true)
    }

    pub async fn delete_latest_scan_batch() -> anyhow::Result<bool> {
        let db = get_db_connection().await?;
        scan_batch::Entity::find()
            .order_by_desc(scan_batch::Column::Id)
            .one(&db)
            .await?
            .map(|model| scan_batch::ActiveModel {
                id: Set(model.id),
                ..Default::default()
            })
            .unwrap()
            .delete(&db)
            .await?;

        Ok(true)
    }
}
