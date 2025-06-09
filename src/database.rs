use mongodb::{Client, bson::Document};

#[derive(Clone)]
pub struct Database {
    pub client: Option<Client>,
}

impl Database {
    pub async fn get_collection(
        &self,
        db_name: &str,
        collection_name: &str,
    ) -> Option<mongodb::Collection<Document>> {
        if let Some(client) = &self.client {
            let database = client.database(db_name);
            let collection = database.collection(collection_name);
            Some(collection)
        } else {
            None
        }
    }

    pub async fn get_collection_gen<T: Send + Sync>(
        &self,
        db_name: &str,
        collection_name: &str,
    ) -> Option<mongodb::Collection<T>> {
        if let Some(client) = &self.client {
            let database = client.database(db_name);
            let collection = database.collection::<T>(collection_name);
            Some(collection)
        } else {
            None
        }
    }
}

pub async fn connect(uri: &str) -> Result<Database, Box<dyn std::error::Error>> {
    let client = Client::with_uri_str(uri).await?;
    Ok(Database {
        client: Some(client),
    })
}
