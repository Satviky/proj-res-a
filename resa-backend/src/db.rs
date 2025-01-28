use mongodb::{Client, options::ClientOptions};

pub async fn connect_to_mongodb() -> mongodb::error::Result<()> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    let client = Client::with_options(client_options)?;

    println!("Connected to MongoDB");
    Ok(())
}
