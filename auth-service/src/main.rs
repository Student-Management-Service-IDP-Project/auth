extern crate env_logger;

use auth_lib::api::authorize::authorize;
use auth_lib::db::mongo::{Database, MongoDB, connect_mongo};

use actix_web::{self, HttpServer, App};
use serde::Deserialize;
use mongodb::{bson::doc, sync::Client};
use actix_cors::Cors;

#[derive(Debug, Deserialize)]
struct Config {
    port: u16,
    host: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Get environment variables
    let config = Config {port:3000, host: "0.0.0.0".into()};

    let db = Database { name: "school".to_string(), collection: "auth".to_string() };

    // Connect to DB
    let _client = connect_mongo();

    // Get Client for DB
    let client: Client;

    match _client {
        Ok(c) => {
            client = c;
        }
        Err(_) => {
            panic!("Could not connect to the Database. Aborting!")
        }
    }
    
    // Initialize State for App
    let _mongo = MongoDB {
        client,
        database: db,
    };

    // Start service
    HttpServer::new(move || {
        let cors = Cors::permissive()
                .allow_any_origin()
                .allow_any_method()
                .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(actix_web::web::Data::new(
                _mongo.clone()
            ))
            .service(authorize())
    }).bind((config.host, config.port))?
    .run()
    .await
}
