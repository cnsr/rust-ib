use crate::utils::validate_credentials;

use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web::error::{ErrorUnauthorized, ErrorBadRequest};

use actix_web::{Error,dev::ServiceRequest, FromRequest, HttpRequest};
use serde::{Serialize, Deserialize};

use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use chrono::{Utc};
use dotenv::dotenv;
use std::env;

const SECS_PER_DAY: i64 = 86400;
const BEARER: &str = "Bearer ";

// claims only have expiration in them for now
#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    exp: usize,
}

// this shit utilizes BearerAuth middleware, can't be arsed to write my own middleware for now lol
pub async fn basic_auth_validator(req: ServiceRequest, _credentials: BearerAuth) -> Result<ServiceRequest, Error> {
    // allow OPTIONS request idk this could help with any CORS issues
    if req.method() == "OPTIONS" {
        return Ok(req);
    }

    let token = req.headers()
        .get("AUTHORIZATION")
        .map(|value| value.to_str().ok())
        .unwrap();

    match token {
        Some(t) => {
          // check that the token is valid - up to you how you do this
          verify_token(&t)?; // this returns an Unauthorised result if the token is invalid
          return Ok(req);
        },
        None => Err(ErrorUnauthorized("JWT token is broken."))
      }
}

fn verify_token(token: &str) -> Result<Claims, Error> {
    if !token.starts_with(BEARER) {
        return Err(ErrorBadRequest("JWT token is not using Bearer prefix."));
    }
    
    let jwt = token.trim_start_matches(BEARER).to_owned();
    
    let decoded = decode::<Claims>(
        &jwt,
        &DecodingKey::from_secret(get_jwt_secret().as_bytes()),
        &Validation::new(Algorithm::HS512),
    )
    .map_err(|_| ErrorUnauthorized("Invalid JWT token."))?;
    
    Ok(decoded.claims)
}

pub fn create_jwt() -> Result<String, Error> {
    dotenv().ok();
    
    // JWT is valid for a whole day - this is NOT SECURE.
    // I just can't be arsed to implement legin access/refresh tokens,
    // especially since i don't have literally anythin to put into the claims
    
    let expiration = Utc::now()
    .checked_add_signed(chrono::Duration::seconds(SECS_PER_DAY))
    .expect("valid timestamp")
    .timestamp();
    
    // just how secure can a jwt with no claims can be
    let claims = Claims {
        exp: expiration as usize,
    };
    let header = Header::new(Algorithm::HS512);
    let key = EncodingKey::from_secret(get_jwt_secret().as_bytes());
    encode(&header, &claims, &key)
    .map_err(|_| ErrorUnauthorized("Failed to create JWT token."))
}

fn get_jwt_secret() -> String {
    dotenv().ok();
    return env::var("JWT_SECRET").expect("JWT_SECRET is not set in .env file.");
}