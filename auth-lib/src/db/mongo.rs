use bson::Document;
use mongodb::{
    bson::doc,
    options::{ClientOptions, Credential, ServerAddress, ServerApi, ServerApiVersion},
    sync::Client,
};
use serde::Deserialize;

pub(super) mod utils {
    pub const MONGO_PORT: &str = "MONGO_PORT";
    pub const MONGO_USERNAME: &str = "MONGO_USERNAME";
    pub const MONGO_PASSWORD: &str = "MONGO_PASSWORD";
    pub const MONGO_DATABASE: &str = "MONGO_DATABASE";
    pub const MONGO_SERVER: &str = "MONGO_SERVER";
    pub const DEFAULT_MONGO_PORT: u16 = 27017;
}

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
    pub name: String,
    pub collection: String,
}

/// Connect to MongoDB cluster based on given url
pub fn connect_mongo() -> mongodb::error::Result<Client> {
    let credential = Credential::builder()
        .username(std::env::var(utils::MONGO_USERNAME).unwrap_or_default())
        .password(std::env::var(utils::MONGO_PASSWORD).unwrap_or_default())
        .source(std::env::var(utils::MONGO_DATABASE).unwrap_or_default())
        .build();

    // Get server addr.
    let server_addr = ServerAddress::Tcp {
        host: std::env::var(utils::MONGO_SERVER).unwrap_or_default(),
        port: Some(
            std::env::var(utils::MONGO_PORT)
                .unwrap_or_default()
                .parse::<u16>()
                .unwrap_or(utils::DEFAULT_MONGO_PORT),
        ),
    };

        // Build client options.
        let options = ClientOptions::builder()
        .hosts(vec![server_addr])
        .credential(credential)
        .build();

    // Get a handle to the cluster
    let client = Client::with_options(options)?;

    // Ping the server to see if it connects to cluster
    client
        .database("school")
        .run_command(doc! {"ping": 1}, None)?;
    println!("Pinged your deployment. You successfully connected to MongoDB!");

    Ok(client)
}

/// Wrapper for filter to query in Database for minimum an instance of desired document
pub fn find_one(_mongodb: &MongoDB, filter: impl Into<Option<Document>>) -> bool {
    let _coll = _mongodb
        .client
        .database(&_mongodb.database.name)
        .collection::<User>(&_mongodb.database.collection);
    let _res = _coll.count_documents(filter, None);

    match _res {
        Ok(n) => {
            if n > 0 {
                return true;
            }
            false
        }
        Err(_) => false,
    }
}
