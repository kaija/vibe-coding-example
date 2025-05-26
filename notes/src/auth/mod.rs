use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{rand_core::OsRng, SaltString};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user id
    pub username: String,
    pub exp: usize,
}

pub struct AuthenticatedUser {
    pub id: Uuid,
    pub username: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = &'static str;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let token = request
            .headers()
            .get_one("Authorization")
            .and_then(|header| header.strip_prefix("Bearer "));

        match token {
            Some(token) => match verify_jwt(token) {
                Ok(claims) => {
                    let user_id = Uuid::parse_str(&claims.sub).unwrap_or_default();
                    Outcome::Success(AuthenticatedUser {
                        id: user_id,
                        username: claims.username,
                    })
                }
                Err(_) => Outcome::Error((Status::Unauthorized, "Invalid token")),
            },
            None => Outcome::Error((Status::Unauthorized, "Missing token")),
        }
    }
}

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(password_hash.to_string())
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(hash)?;
    let argon2 = Argon2::default();
    match argon2.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(()) => Ok(true),
        Err(_) => Ok(false),
    }
}

pub fn generate_jwt(user_id: Uuid, username: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        username: username.to_string(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
}

pub fn verify_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}