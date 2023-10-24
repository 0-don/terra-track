use mongodb::{
    bson::{doc, Document},
    Client, Collection,
};

pub async fn connect_db() -> anyhow::Result<()> {
    // Replace the placeholder with your Atlas connection string
    let uri = "mongodb://root:root@127.0.0.1:27017";
    // Create a new client and connect to the server
    let client = Client::with_uri_str(uri).await?;
    // Get a handle on the movies collection
    let database = client.database("sample_mflix");
    let my_coll: Collection<Document> = database.collection("movies");
    // Find a movie based on the title value
    let my_movie = my_coll
        .insert_one(doc! { "title": "The Perils of Pauline" }, None)
        .await?;
    // Print the document
    println!("Found a movie:\n{:#?}", my_movie);

    Ok(())
}
