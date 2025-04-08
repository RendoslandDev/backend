use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // username
    pub exp: usize,   // expiry timestamp
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

const DEFAULT_EXPIRATION_HOURS: i64 = 24;

pub fn create_token(username: &str) -> Result<String, String> {
    let secret = env::var("JWT_SECRET").map_err(|_| "mECQyti5RZZ9YzIBuoOeneb1DEr/MAs65Qa7NxYyH0U=".to_string())?;
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(DEFAULT_EXPIRATION_HOURS))
        .expect("Invalid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: username.to_owned(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    ).map_err(|e| e.to_string())
}

pub fn validate_token(token: &str) -> bool {
    let secret = match env::var("JWT_SECRET") {
        Ok(secret) => secret,
        Err(_) => return false,
    };

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    ).is_ok()
}

pub fn authenticate(login: &LoginRequest) -> Result<String, String> {
    // In a real app, verify against your user database
    if login.username == "admin" && login.password == "admin123" {
        create_token(&login.username)
    } else {
        Err("Invalid credentials".to_string())
    }
}