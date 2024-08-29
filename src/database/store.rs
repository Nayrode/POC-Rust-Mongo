use mongodb::{Client, options::ClientOptions, bson::{doc, Document}};
use mongodb::error::Result;
use mongodb::bson::from_document;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Pipeline {
    pub name: String,
    // Add other fields as needed
}

pub async fn get_pipelines(client: Client) -> Result<Vec<Pipeline>> {
    let db = client.database("sealci");
    let collection = db.collection::<Document>("pipelines");

    let mut pipelines = Vec::new();
    let mut cursor = collection.find(Document::new()).await?;
    
    while cursor.advance().await? {
        let document = cursor.current().to_owned();
        let pipeline: Pipeline = from_document(document.to_document()?)?;
        pipelines.push(pipeline);
    }

    Ok(pipelines)
}

pub async fn create_pipeline(client: Client, pipeline: Pipeline) -> Result<()> {
    let db = client.database("sealci");
    let collection = db.collection::<Pipeline>("pipelines");

    collection.insert_one(pipeline).await?;

    Ok(())
}