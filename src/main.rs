use database::store::{create_pipeline, get_pipelines, Pipeline};
use mongodb::{Client, options::ClientOptions};
use std::error::Error;
use tokio;

mod database;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
   // Load the MongoDB connection string from an environment variable:
   let client_uri =
        "mongodb://user:pass@0.0.0.0:27018/?directConnection=true";

   // A Client is needed to connect to MongoDB:
   // An extra line of code to work around a DNS issue on Windows:
   let options =
      ClientOptions::parse(client_uri.to_string()).await?;
   let client = Client::with_options(options)?;

let client_clone = client.clone();
let pipelines = get_pipelines(client_clone).await?;
    println!("Pipelines: {:?}", pipelines);

    let new_pipeline = Pipeline {
        name: "New Pipeline".to_owned(),
        // Set other fields as needed
    };
    create_pipeline(client, new_pipeline).await?;

    Ok(())

}