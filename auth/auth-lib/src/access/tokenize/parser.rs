use actix_web::web;
use chrono::{Utc, Duration};
use jsonwebtoken::{ encode, EncodingKey, Header};
use crate::access::{extractor::extract::{AccessClaims, Token, RefreshClaims, Info}, tokens::Secret};

/// Encode JWT access token based on username, name and secret from app's shared data
pub fn encode_access_token(username: String, name: String, secret: &web::Data<Secret>) -> String {
    // Should expire in a short time
    let exp: usize = (Utc::now() + Duration::minutes(5)).timestamp() as usize;

    let claims = AccessClaims {
        exp,
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

/// Encode JWT access token based on username and secret from app's shared data
pub fn encode_refresh_token(username: String, secret: &web::Data<Secret>) -> String {
    // Should take longer than the access token to expire
    let exp: usize = (Utc::now() + Duration::days(7)).timestamp() as usize;

    let claims = RefreshClaims {
        exp,
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
