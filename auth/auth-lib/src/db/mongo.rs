use bson::Document;
use serde::Deserialize;
use mongodb::{bson::doc, options::{ClientOptions, ServerApi, ServerApiVersion}, sync::Client};

use super::parser::user::User;
extern crate env_logger;

/// Struct for Database manipulation
#[derive(Debug, Clone)]
pub struct MongoDB {
    pub client: Client,
    // Specific database to be accessed
    pub database: Database,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Database {
    pub url: String,
    pub name: String,
    pub collection: String,
}

/// Connect to MongoDB cluster based on given url
pub fn connect_mongo(url: &String) -> mongodb::error::Result<Client> {
    let mut client_options = ClientOptions::parse(url)?;
    
    // Set the server_api field of the client_options object to Stable API version 1
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);
    
    // Get a handle to the cluster
    let client = Client::with_options(client_options)?;
    
    // Ping the server to see if it connects to cluster
    client
        .database("auth-db")
        .run_command(doc! {"ping": 1}, None)?;
    println!("Pinged your deployment. You successfully connected to MongoDB!");
    
    Ok(client)
}

/// Wrapper for filter to query in Database for minimum an instance of desired document
pub fn find_one(_mongodb: &MongoDB, filter: impl Into<Option<Document>>) -> bool {
    let _coll = _mongodb.client.database(&_mongodb.database.name).collection::<User>(&_mongodb.database.collection);
    let _res = _coll.count_documents(filter, None);

    match _res {
        Ok(n) => {
            if n > 0 {
                return true;
            }
            false
        }
        Err(_) => { false }
    }
}