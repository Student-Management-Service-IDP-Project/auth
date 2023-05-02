use std::io::Error;

use actix_web::{Scope, web, HttpResponse, http::StatusCode, HttpRequest};
use bson::doc;
use jsonwebtoken::{ encode, EncodingKey, Header, DecodingKey, Validation, TokenData, decode };
use serde::{Deserialize, Serialize};
use crate::{db::{mongo::{MongoDB, find_one}, parser::user::{User, DBParser}}, access::tokenize::parser::encode_refresh_token};
use uuid::Uuid;
extern crate argon2;

#[allow(unused_imports)]
use super::super::access::tokens::Secret;

#[derive(Deserialize, Serialize, Debug)]
pub struct RegisterForm {
    pub username: String,
    pub email: String,
    pub password: String,

    pub name: Option<String>,    
    pub photo_url: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

pub trait Authorize {
    fn verify_pwsh(&self, hash: &String) -> bool;
    fn generate_refresh(&self) -> String;
}

impl Authorize for LoginForm {
    fn verify_pwsh(&self, hash: &String) -> bool {
        let _pws = self.password.clone();

        argon2::verify_encoded(&hash, &_pws.as_bytes()).unwrap()
    }

    fn generate_refresh(&self) -> String {
        todo!()
    }
}

pub trait Generate {
    fn generate_pwsh(&self) -> String;
}

impl Generate for RegisterForm {
    fn generate_pwsh(&self) -> String {
        let _pws = self.password.clone();
        let salt = b"randomsalt";

        let config = argon2::Config::default();

        argon2::hash_encoded(&_pws.as_bytes(), salt, &config).unwrap_or("".to_string())
    }
}


pub fn authorize() -> Scope {
    let secret = envy::prefixed("SECRET__")
    .from_env::<Secret>().expect("Please provide SECRET__ACCESS and SECRET__REFRESH in .env");

    web::scope("/user")
        .app_data(web::Data::new(secret.clone()))
        .route("/login", web::post().to(login))
        .route("/register", web::post().to(register))
        .route("/validate", web::post().to(validate))
        .route("/refresh", web::post().to(refresh))
}

async fn register(req: HttpRequest, form: web::Form<RegisterForm>) -> HttpResponse {
    let data = req.app_data::<web::Data<MongoDB>>().unwrap().database.clone();
    let client = req.app_data::<web::Data<MongoDB>>().unwrap().client.clone();

    // Get secret from local data
    let _secret = req.app_data::<web::Data<Secret>>();

    match _secret {
        Some(_) => {}
        None => {
            return HttpResponse::new(StatusCode::BAD_REQUEST);
        }
    }

    let _mongodb = MongoDB {
        client: client,
        database: data,
    };

    // Check if username or email already exist in DB
    let _username = form.username.clone();
    let _email = form.email.clone();

    let filter_email = doc! {
            "email": _email
    };

    
    let filter_username = doc! {
        "username": _username
    };
    
    if find_one(&_mongodb, filter_email) || find_one(&_mongodb, filter_username) {
        return HttpResponse::BadRequest().body(format!("Username or Email already in use!"));
    }

    // Create a new user
    let _new_user = User {
        uuid: Uuid::new_v4(),
        username: form.username.clone(),
        email: form.email.clone(),
        password_hash: form.generate_pwsh(),
        name: form.name.clone(),
        refresh_token: encode_refresh_token(form.username.clone(), _secret.unwrap()),
        //refresh_token: "".to_string(),
        photo_url: form.photo_url.clone(),
        refresh_creation: chrono::offset::Utc::now(),
    };

    // Insert the user in the database
    let _db = _new_user.insert(&_mongodb);

    match _db {
        Ok(_) => {
            HttpResponse::Ok().body(format!("Created user: {:#?}", _new_user))
        }
        Err(_) => {
            HttpResponse::BadRequest().body(format!("Could not create user!"))
        }
    }
}

async fn login(req: HttpRequest, form: web::Form<LoginForm>) -> HttpResponse {
    let data = req.app_data::<web::Data<MongoDB>>().unwrap().database.clone();
    let client = req.app_data::<web::Data<MongoDB>>().unwrap().client.clone();

    // Get secret from local data
    let _secret = req.app_data::<web::Data<Secret>>();

    match _secret {
        Some(_) => {}
        None => {
            return HttpResponse::new(StatusCode::BAD_REQUEST);
        }
    }

    let _mongodb = MongoDB {
        client: client,
        database: data,
    };

    // Check if username exists in DB
    let _username = form.username.clone();

    let filter_username = doc! {
        "username": _username
    };

    if !find_one(&_mongodb, filter_username.clone()) {
        return HttpResponse::BadRequest().body(format!("Username not found!"));
    }
    let _coll = _mongodb.client.database(&_mongodb.database.name).collection::<User>(&_mongodb.database.collection);
    let _cursor = _coll.find(filter_username.clone(), None).unwrap();
    let _user = _cursor.deserialize_current().unwrap();
    
    let _password_hash = _user.password_hash.clone();
    
    if !form.verify_pwsh(&_password_hash) {
        return HttpResponse::BadRequest().body(format!("Password doesn't match!"));   
    }

    let _password = form.password.clone();

    HttpResponse::new(StatusCode::OK)
}

async fn refresh() -> HttpResponse {
    HttpResponse::new(StatusCode::OK)
}

async fn validate() -> HttpResponse {
    HttpResponse::new(StatusCode::OK)
}