use entity::ip_main;
use migration::db::get_db_connection;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, Set, TryIntoModel,
};

pub struct Mutation;

impl Mutation {
    pub async fn create_ip_main(
        active_model: ip_main::ActiveModel,
    ) -> anyhow::Result<ip_main::Model> {
        let db = get_db_connection().await?;
        let model = active_model.save(&db).await?.try_into_model()?;

        Ok(model)
    }

    pub async fn upsert_ip_main(
        active_model: ip_main::ActiveModel,
    ) -> anyhow::Result<ip_main::Model> {
        let db = get_db_connection().await?;
        let mut model = ip_main::Entity::find()
            .filter(ip_main::Column::IpAddress.eq(active_model.ip_address.as_ref().to_owned()))
            .one(&db)
            .await?
            .map(|model| model.try_into_model())
            .transpose()?;

        if model.is_none() {
            model = active_model.insert(&db).await.ok();
        }

        Ok(model.unwrap())
    }

    pub async fn upsert_ip_main_by_ip(
        ip: &String,
        ip_type: &String,
    ) -> anyhow::Result<ip_main::Model> {
        println!("upsert_ip_main_by_ip: {}", ip);
        let db = get_db_connection().await?;
        let mut model = ip_main::Entity::find()
            .filter(ip_main::Column::IpAddress.eq(ip))
            .one(&db)
            .await?
            .map(|model| model.try_into_model())
            .transpose()?;

        if model.is_none() {
            model = ip_main::ActiveModel {
                ip_type: Set(ip_type.to_string()),
                ip_address: Set(ip.to_string()),
                registry: Set("".to_string()),
                ..Default::default()
            }
            .insert(&db)
            .await
            .ok();
        }

        Ok(model.unwrap())
    }

    pub async fn update_ip_main(id: i64, model: ip_main::Model) -> anyhow::Result<ip_main::Model> {
        let db = get_db_connection().await?;
        let model = ip_main::ActiveModel {
            id: Set(id),
            ..model.into()
        }
        .save(&db)
        .await?
        .try_into_model()?;

        Ok(model)
    }

    pub async fn delete_ip_main(id: i64) -> anyhow::Result<bool> {
        let db = get_db_connection().await?;
        ip_main::ActiveModel {
            id: Set(id),
            ..Default::default()
        }
        .delete(&db)
        .await?;

        Ok(true)
    }

    pub async fn delete_all_ip_main() -> anyhow::Result<bool> {
        let db = get_db_connection().await?;
        ip_main::Entity::delete_many().exec(&db).await?;

        Ok(true)
    }

    pub async fn delete_latest_ip_main() -> anyhow::Result<bool> {
        let db = get_db_connection().await?;
        ip_main::Entity::find()
            .order_by_desc(ip_main::Column::Id)
            .one(&db)
            .await?
            .map(|model| ip_main::ActiveModel {
                id: Set(model.id),
                ..Default::default()
            })
            .unwrap()
            .delete(&db)
            .await?;

        Ok(true)
    }
}
