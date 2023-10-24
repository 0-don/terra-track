use mongodb::{
    bson::{doc, Document},
    Client, Collection,
};
use std::env;

pub const TABLE_IPS: &'static str = "ips";

pub async fn connect_db() -> anyhow::Result<()> {
    let uri = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db = env::var("MONGO_DB").expect("MONGO_DB is not set in .env file");

    let client = Client::with_uri_str(uri).await?;

    let database = client.database(db.as_str());
    let my_coll: Collection<Document> = database.collection(TABLE_IPS);

    let _ = my_coll
        .insert_one(doc! { "title": "The Perils of Pauline" }, None)
        .await?;

    Ok(())
}
