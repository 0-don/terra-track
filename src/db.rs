use mongodb::{
    bson::{doc, Document},
    Client, Collection,
};

pub async fn connect_db() -> anyhow::Result<()> {
    let uri = "mongodb://root:root@127.0.0.1:27017";

    let client = Client::with_uri_str(uri).await?;

    let database = client.database("sample_mflix");
    let my_coll: Collection<Document> = database.collection("movies");

    let _ = my_coll
        .insert_one(doc! { "title": "The Perils of Pauline" }, None)
        .await?;

    // println!("Found a movie:\n{:#?}", my_movie);

    Ok(())
}
