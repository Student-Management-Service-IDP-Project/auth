use actix_web::{web};
use chrono::{Utc, Duration};
use jsonwebtoken::{ encode, EncodingKey, Header};

use crate::{access::{extractor::extract::{AccessClaims, Token, RefreshClaims, Info}, tokens::Secret}, db::parser::user::User};

pub fn encode_access_token(username: String,
    name: String,
    secret: &web::Data<Secret>) -> String {
    let exp: usize = (Utc::now() + Duration::minutes(5)).timestamp() as usize; /* exp date -> tbd with refresh times */

    let claims = AccessClaims {
        exp: exp,
        custom: Token {
            username: username.clone(),
            name: name.clone(),
        }
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.access.as_str().as_ref())
    ).unwrap()
}

pub fn encode_refresh_token(username: String, secret: &web::Data<Secret>) -> String {
    let exp: usize = (Utc::now() + Duration::days(7)).timestamp() as usize; /* exp date -> tbd with refresh times */

    let claims = RefreshClaims {
        exp: exp,
        id: Info {
            username: username.clone()
        }
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.refresh.as_str().as_ref())
    ).unwrap()
}
