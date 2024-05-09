extern crate chrono;

use chrono::Utc;
use serde::{Deserialize, Serialize, };
use mongodb::{bson::doc, results::{InsertOneResult, DeleteResult, UpdateResult}};
use uuid::Uuid;

use crate::db::mongo::MongoDB;

pub trait DBParser {
    fn insert(&self, mongodb: &MongoDB) -> Result<InsertOneResult, mongodb::error::Error>;
    fn delete(&self, mongodb: &MongoDB) -> Result<DeleteResult, mongodb::error::Error>;
    
    fn update_name(&self, mongodb: &MongoDB, name: String) -> Result<UpdateResult, mongodb::error::Error>;
    fn update_password(&self, mongodb: &MongoDB, password_hash: String) -> Result<UpdateResult, mongodb::error::Error>;
    fn update_token(&self, mongodb: &MongoDB, refresh_token: String) -> Result<UpdateResult, mongodb::error::Error>;
}

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    #[serde(rename = "_id")]
    pub uuid: Uuid,

    pub username: String,
    pub email: String,

    pub password_hash: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub refresh_token: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_url: Option<String>,

    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub refresh_creation: chrono::DateTime<Utc>,
}

impl DBParser for User {
    fn insert(&self, mongodb: &MongoDB) -> Result<InsertOneResult, mongodb::error::Error> {
        let _coll = mongodb.client.database(&mongodb.database.name).collection::<User>(&mongodb.database.collection);
        _coll.insert_one(self, None)
    }

    fn delete(&self, mongodb: &MongoDB) -> Result<DeleteResult, mongodb::error::Error> {
        let _coll = mongodb.client.database(&mongodb.database.name).collection::<User>(&mongodb.database.collection);

        let _uuid = self.uuid;

        let filter = doc! {
            "_id": _uuid
        };

        _coll.delete_one(filter, None)
    }

    fn update_name(&self, mongodb: &MongoDB, name: String) -> Result<UpdateResult, mongodb::error::Error> {
        let _coll = mongodb.client.database(&mongodb.database.name).collection::<User>(&mongodb.database.collection);
        
        let _uuid = self.uuid;

        let filter = doc! {
            "_id": _uuid
        };

        let update = doc! {
            "$set": {
                "name": name
            }
        };

        _coll.update_one(filter, update, None)
    }

    fn update_password(&self, mongodb: &MongoDB, password_hash: String) -> Result<UpdateResult, mongodb::error::Error> {
        let _coll = mongodb.client.database(&mongodb.database.name).collection::<User>(&mongodb.database.collection);
        
        let _uuid = self.uuid;

        let filter = doc! {
            "_id": _uuid
        };

        let update = doc! {
            "$set": {
                "password_hash": password_hash
            }
        };

        _coll.update_one(filter, update, None)
    }

    fn update_token(&self, mongodb: &MongoDB, refresh_token: String) -> Result<UpdateResult, mongodb::error::Error> {
        let _coll = mongodb.client.database(&mongodb.database.name).collection::<User>(&mongodb.database.collection);
        
        let _uuid = self.uuid;

        let filter = doc! {
            "_id": _uuid
        };

        let update = doc! {
            "$set": {
                "refresh_token": refresh_token
            }
        };

        _coll.update_one(filter, update, None)
    }
}