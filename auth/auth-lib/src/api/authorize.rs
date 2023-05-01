#[allow(unused_imports)]
use actix_web::{Scope, web, Handler, HttpResponse, http::StatusCode, Responder, HttpRequest};
use chrono::NaiveDateTime;
#[allow(unused_imports)]
use chrono::{Utc, Duration};
#[allow(unused_imports)]
use jsonwebtoken::{ encode, EncodingKey, Header, DecodingKey, Validation, TokenData, decode };
use serde::{Deserialize, Serialize};
use crate::db::{mongo::MongoDB, parser::user::{User, DBParser}};
use uuid::Uuid;

#[allow(unused_imports)]
use super::super::access::tokens::Secret;

#[derive(Deserialize, Serialize, Debug)]
pub struct UserForm {
    pub username: String,
    pub email: String,
    pub password: String,

    pub name: Option<String>,    
    pub photo_url: Option<String>,
}

pub fn authorize() -> Scope {
    let secret = envy::prefixed("SECRET__")
    .from_env::<Secret>().expect("Please provide SECRET__ACCESS and SECRET__REFRESH in .env");

    web::scope("/user")
        .app_data(web::Data::new(secret.clone()))
        .route("/login", web::post().to(login))
        .route("/register", web::get().to(register))
        .route("/validate", web::post().to(validate))
        .route("/refresh", web::get().to(refresh))
}

#[derive(Deserialize, Serialize, Debug)]
struct Info {
    id: String,
}

async fn login(req: HttpRequest, form: web::Form<UserForm>) -> HttpResponse {
    let data = req.app_data::<web::Data<MongoDB>>().unwrap().database.clone();
    let client = req.app_data::<web::Data<MongoDB>>().unwrap().client.clone();

    let _mongodb = MongoDB {
        client: client,
        database: data,
    };

    let _new_user = User {
        uuid: Uuid::new_v4(),
        username: form.username.clone(),
        email: form.email.clone(),
        password_hash: form.password.clone(),
        name: form.name.clone(),
        refresh_token: "Some Refresh Token".to_string(),
        photo_url: form.photo_url.clone(),
        refresh_creation: chrono::offset::Utc::now(),
    };

    let _db = _new_user.insert(&_mongodb);

    match _db {
        Ok(_) => {
            HttpResponse::Ok().body(format!("User: {:#?}", _new_user))
        }
        Err(_) => {
            HttpResponse::new(StatusCode::BAD_REQUEST)
        }
    }
}

async fn register() -> HttpResponse {
    HttpResponse::new(StatusCode::OK)
}

async fn refresh() -> HttpResponse {
    HttpResponse::new(StatusCode::OK)
}

async fn validate() -> HttpResponse {
    HttpResponse::new(StatusCode::OK)
}