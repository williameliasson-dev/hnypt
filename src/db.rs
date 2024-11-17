use dotenv::dotenv;
use mongodb::Client;
use once_cell::sync::OnceCell;
use std::env;

static CLIENT: OnceCell<Client> = OnceCell::new();

pub struct MongoDB;

impl MongoDB {
    pub async fn init() -> mongodb::error::Result<()> {
        dotenv().ok();

        let uri = env::var("MONGODB_URI").expect("MONGODB_URI not set");
        let client = Client::with_uri_str(&uri).await?;
        CLIENT.set(client).unwrap();
        Ok(())
    }

    pub fn get_client() -> &'static Client {
        CLIENT.get().expect("MongoDB client not initialized")
    }
}
