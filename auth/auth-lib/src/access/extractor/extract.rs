use serde::{Serialize, Deserialize};
use jsonwebtoken::{decode, Algorithm, Validation, DecodingKey};
use actix_web::{http::{self}, error, web, FromRequest};
use std::future::{Ready, ready};
use super::super::tokens::Secret;

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessClaims {
    pub exp: usize,          // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    pub custom: Token,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub username: String,
    pub name: String,
}

/* Trait to validate and retrieve User Data from JWT */
impl FromRequest for Token {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;
    
    fn from_request(req: &actix_web::HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        /* Get Header from Request */
        let _header = req.headers().get(http::header::AUTHORIZATION);

        match _header {
            Some(_) => {}
            None => {
                return ready(Err(error::ErrorUnauthorized("Authorization header missing!")));
            }
        }

        /* Get JWT from Header */
        let _token = _header.unwrap().to_str().unwrap_or("").to_string();
        if _token.is_empty() {
            return ready(Err(error::ErrorUnauthorized("Empty access token provided!")));
        }
        
        /* Get Access Secret from app_data */
        let _secret = req.app_data::<web::Data<Secret>>();

        match _secret {
            Some(_) => {}
            None => {
                return ready(Err(error::ErrorBadRequest("Missing secret key!")));
            }
        }

        let access_secret = _secret.unwrap().access.as_str();
        
        let data = decode::<AccessClaims>(
            &_token,
            &DecodingKey::from_secret(access_secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        );
        
        /* Decode the token return */
        match data {
            Ok(t) => {
                return ready(
                    Ok(
                        Token {
                            username: t.claims.custom.username,
                            name: t.claims.custom.name,
                        }
                    )
                );
            }
            Err(_) => {
                return ready(Err(error::ErrorUnauthorized("Invalid token!")));
                }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshClaims {
    pub exp: usize,         // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    pub id: Info,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    pub username: String,
}

/* Trait to validate and retrieve User Data from JWT */
impl FromRequest for Info {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;
    
    fn from_request(req: &actix_web::HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        /* Get Header from Request */
        let _header = req.headers().get(http::header::AUTHORIZATION);

        match _header {
            Some(_) => {}
            None => {
                return ready(Err(error::ErrorUnauthorized("Authorization header missing!")));
            }
        }

        /* Get JWT from Header */
        let _token = _header.unwrap().to_str().unwrap_or("").to_string();
        if _token.is_empty() {
            return ready(Err(error::ErrorUnauthorized("Empty access token provided!")));
        }
        
        /* Get Access Secret from app_data */
        let _secret = req.app_data::<web::Data<Secret>>();

        match _secret {
            Some(_) => {}
            None => {
                return ready(Err(error::ErrorBadRequest("Missing secret key!")));
            }
        }

        let access_secret = _secret.unwrap().refresh.as_str();
        
        let data = decode::<RefreshClaims>(
            &_token,
            &DecodingKey::from_secret(access_secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        );
        
        /* Decode the token return */
        match data {
            Ok(t) => {
                return ready(
                    Ok(
                        Info {
                            username: t.claims.id.username,
                        }
                    )
                );
            }
            Err(_) => {
                return ready(Err(error::ErrorUnauthorized("Invalid token!")));
                }
        }
    }
}