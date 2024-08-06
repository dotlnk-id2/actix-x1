use mongodb::{
    bson::{doc, oid::ObjectId, DateTime, Document},
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client,
};

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let mut client_options = ClientOptions::parse("mongodb+srv://whoami:frankson1@mg-jp-n1.xvwgqlk.mongodb.net/?retryWrites=true&w=majority&appName=mg-jp-n1").await?;

    // Set the server_api field of the client_options object to set the version of the Stable API on the client
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    // Get a handle to the cluster
    let client = Client::with_options(client_options)?;

    // Ping the server to see if you can connect to the cluster
    client.database("gw").run_command(doc! {"ping": 1}).await?;
    println!("Pinged your deployment. You successfully connected to MongoDB!");

    for name in client.list_database_names().await? {
        println!("- {}", name);
    }

    let doc: mongodb::Collection<Document> = client.database("gw").collection("test_doc");

    let mut docs = Vec::new();
    for i in 1..=100_000_000 {
        docs.push(doc! {
            "_id":ObjectId::new(),
            "key": i,
            "create_time":DateTime::now(),
        });

        if i % 10_000 == 0 {
            doc.insert_many(docs.clone()).await?;
            println!("insert to i={}", i);
            docs.clear();
        }
    }

    Ok(())
}
