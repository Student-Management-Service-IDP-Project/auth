use actix_web::{Scope, web, HttpResponse, http::StatusCode, HttpRequest};
use bson::doc;
use jsonwebtoken::{ DecodingKey, Validation, TokenData, decode };
use serde::{Deserialize, Serialize};
use crate::{db::{mongo::{MongoDB, find_one}, parser::user::{User, DBParser}}, access::{tokenize::parser::{encode_refresh_token, encode_access_token}, extractor::extract::RefreshClaims}};
use uuid::Uuid;
extern crate argon2;

#[allow(unused_imports)]
use super::super::access::tokens::Secret;

/// Should be passed from the Application's FrontEnd microservice
#[derive(Deserialize, Serialize, Debug)]
pub struct RegisterForm {
    pub username: String,
    pub email: String,
    pub password: String,

    pub name: Option<String>,    
    pub photo_url: Option<String>,
}

/// Should be passed from the Application's FrontEnd microservice
#[derive(Deserialize, Serialize, Debug)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

/// Trait for password verification against hash using bcrypt algorithm
pub trait Authorize {
    fn verify_pwsh(&self, hash: &String) -> bool;
}

impl Authorize for LoginForm {
    fn verify_pwsh(&self, hash: &String) -> bool {
        let _pws = self.password.clone();

        argon2::verify_encoded(&hash, &_pws.as_bytes()).unwrap()
    }
}

/// Trait for password hash to be stored in database bcrypt algorithm
pub trait Generate {
    fn generate_pwsh(&self, salt: &[u8]) -> String;
}

impl Generate for RegisterForm {
    fn generate_pwsh(&self, salt: &[u8]) -> String {
        let _pws = self.password.clone();

        let config = argon2::Config::default();

        argon2::hash_encoded(&_pws.as_bytes(), salt, &config).unwrap_or("".to_string())
    }
}

#[derive(Serialize,Deserialize)]
struct LoginResponse {
    access_token: String,
    refresh_token: String,
}

#[derive(Serialize,Deserialize)]
struct Response {
    message: String,
}

#[derive(Serialize, Deserialize)]
struct RefreshForm {
    token: String,
}

#[derive(Serialize, Deserialize)]
struct RefreshResponse {
    access_token: String,
}

#[derive(Serialize, Deserialize)]
struct ValidateResponse {
    username: String,
    name: String,
}

pub fn authorize() -> Scope {
    let secret = envy::prefixed("SECRET__")
    .from_env::<Secret>().expect("Please provide SECRET__ACCESS, SECRET__REFRESH and SECRET__SALT in .env");

    web::scope("/user")
        .app_data(web::Data::new(secret.clone()))
        .route("/login", web::post().to(login))
        .route("/register", web::post().to(register))
        .route("/validate", web::get().to(validate))
        .route("/refresh", web::post().to(refresh))
}

/// Register should add user from POST form to database and return 200 OK
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
        password_hash: form.generate_pwsh(_secret.unwrap().salt.as_bytes()),
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
            HttpResponse::Ok().json(
                Response {
                    message: format!("Hello, {:#?}!", _new_user.name.clone())
                }
            )
        }
        Err(_) => {
            HttpResponse::BadRequest().body(format!("Could not create user!"))
        }
    }
}

/// Login should check user's identity from POST form return access and refresh tokens
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

    // Verify password with hash
    let _coll = _mongodb.client.database(&_mongodb.database.name).collection::<User>(&_mongodb.database.collection);
    let _cursor_user = _coll.find(filter_username.clone(), None).unwrap();
    let _user = _cursor_user.deserialize_current().unwrap();
    
    let _password_hash = _user.password_hash.clone();
    
    if !form.verify_pwsh(&_password_hash) {
        return HttpResponse::BadRequest().body(format!("Password doesn't match!"));   
    }

    // Return access and refresh JWT
    let _refresh_jwt = _user.refresh_token.clone();

    // Decode Token to verify if it expired
    let decoded: Result<TokenData<RefreshClaims>, jsonwebtoken::errors::Error> = decode::<RefreshClaims>(
        &_refresh_jwt,
        &DecodingKey::from_secret(_secret.unwrap().refresh.as_str().as_ref()),
        &Validation::new(jsonwebtoken::Algorithm::HS256)
    );

    match decoded {
        Ok(_) => {},
        Err(err) => {
            // Refresh JWT has expired, generate a new one
            if let jsonwebtoken::errors::ErrorKind::ExpiredSignature = err.clone().into_kind() {
                let _new_token = encode_refresh_token(form.username.clone(), &_secret.unwrap());

                match _user.update_token(&_mongodb, _new_token) {
                    Ok(_) => {},
                    Err(_) => {
                        return HttpResponse::BadRequest().json(Response {message: String::from("Error while updating the refresh token in Database")});
                    }
                }
            }
            return HttpResponse::BadRequest().json(Response {message: format!("Could not parse Refresh Token because of {:?}", err.clone().into_kind())});   
        }
    }

    let access_token = encode_access_token(_user.username.clone(), _user.name.unwrap_or("".to_string()).clone(), &_secret.unwrap());
    let refresh_token: String = _user.refresh_token.clone();

    HttpResponse::Ok().json(
        LoginResponse {
            access_token,
            refresh_token
        }
    )
}

/// Silent refresh should find token's user in database and respond with a new access token
async fn refresh(req: HttpRequest, form: web::Form<RefreshForm>) -> HttpResponse {
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

    let filter_token = doc! {
        "refresh_token": form.token.clone()
    };

    if !find_one(&_mongodb, filter_token.clone()) {
        return HttpResponse::BadRequest().json(Response { message: String::from("Refresh Token doesn't exist. Please redirect user to login.")})
    }

    let _coll = _mongodb.client.database(&_mongodb.database.name).collection::<User>(&_mongodb.database.collection);
    let _cursor_user = _coll.find(filter_token.clone(), None).unwrap();

    let _user = _cursor_user.deserialize_current().unwrap();

    let access_token = encode_access_token(_user.username.clone(), _user.name.unwrap_or("".to_string()).clone(), _secret.unwrap());

    HttpResponse::Ok().json(
        RefreshResponse {
            access_token
        }
    )
}

// Validation for Access Token
async fn validate(token: crate::access::extractor::extract::Token) -> HttpResponse {
    HttpResponse::Ok().json(
        ValidateResponse {
            username: token.username,
            name: token.name,
        }
    )
}
