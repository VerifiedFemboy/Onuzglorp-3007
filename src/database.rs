use mongodb::{bson, error::Result, options::ClientOptions, Client};

pub struct Database {
    client: Option<Client>,
    connection: String,
}

impl Database {
    
    pub fn new(connection: String) -> Self {
        Self {
            client: None,
            connection,
        }
    }

    pub async fn connect(&self) -> Result<Self> {
        let client_options = ClientOptions::parse(&self.connection).await.expect("Failed to parse client options");
        let client = Client::with_options(client_options).expect("Failed to create client");
        Ok(Self {
            client: Some(client),
            connection: self.connection.clone(),
        })
    }

    #[allow(dead_code)]
    pub fn get_database(&self, database_name: &str) -> Option<mongodb::Database> {
        match &self.client {
            Some(client) => {
                Some(client.database(database_name))
            },
            None => None,
        }
    }

    pub fn get_collection(&self, database_name: &str, collection_nanme: &str) -> Option<mongodb::Collection<bson::Document>> {
        match &self.client {
            Some(client) => {
                Some(client.database(database_name).collection(collection_nanme))
            },
            None => None,
        }
        
    }
}