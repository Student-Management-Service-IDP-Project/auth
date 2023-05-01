use bson::Document;
use serde::{Deserialize, Deserializer};
use mongodb::{bson::doc, options::{ClientOptions, ServerApi, ServerApiVersion}, sync::{Client, self}, Collection};
extern crate env_logger;

#[derive(Debug, Deserialize, Clone)]
pub struct Database {
    pub url: String,
    pub name: String,
    pub collection: String,
}

#[derive(Debug, Clone)]
pub struct MongoDB {
    pub client: Client,
    pub database: Database,
}

pub fn connect_mongo(url: &String) -> mongodb::error::Result<Client> {
    let mut client_options = ClientOptions::parse(url)?;
    
    // Set the server_api field of the client_options object to Stable API version 1
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);
    
    // Get a handle to the cluster
    let client = Client::with_options(client_options)?;
    
    // Ping the server to see if you can connect to the cluster
    client
        .database("auth-db")
        .run_command(doc! {"ping": 1}, None)?;
    println!("Pinged your deployment. You successfully connected to MongoDB!");
    
    Ok(client)
}