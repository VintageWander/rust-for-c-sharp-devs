use crate::Result;
use mongodb::{options::ClientOptions, Client, Collection};

pub struct Mongo {
    client: Client,
}

impl Mongo {
    pub async fn init() -> Result<Self> {
        let mongodb_uri = std::env::var("MONGODB_URI")?;
        let mut client_options = ClientOptions::parse(&mongodb_uri).await?;
        client_options.app_name = Some("rust-for-csharp-dev".to_string());
        Ok(Self {
            client: Client::with_options(client_options)?,
        })
    }

    pub fn get_collection<T>(&self, coll_name: &str) -> Collection<T> {
        self.client
            .database("rust-for-csharp-dev")
            .collection(coll_name)
    }
}
