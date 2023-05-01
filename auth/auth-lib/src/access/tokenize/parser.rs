use actix_web::{web};
use chrono::{Utc, Duration};
use jsonwebtoken::{ encode, EncodingKey, Header};

use crate::{access::extractor::extract::{AccessClaims, Token, RefreshClaims, Info}, db::parser::user::User};

pub fn encode_access_token(user: &User, secret: web::Data<String>) -> String {
    let exp: usize = (Utc::now() + Duration::minutes(5)).timestamp() as usize; /* exp date -> tbd with refresh times */

    let claims = AccessClaims {
        exp: exp,
        custom: Token {
            username: user.username.clone(),
            name: user.name.clone(),
            photo_url: user.photo_url.clone()
        }
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_str().as_ref())
    ).unwrap()
}

pub fn encode_refresh_token(user: &User, secret: web::Data<String>) -> String {
    let exp: usize = (Utc::now() + Duration::minutes(5)).timestamp() as usize; /* exp date -> tbd with refresh times */

    let claims = RefreshClaims {
        exp: exp,
        id: Info {
            username: user.username.clone()
        }
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_str().as_ref())
    ).unwrap()
}
