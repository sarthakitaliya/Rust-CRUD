use chrono::{Duration, Utc};

use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use store::{models::user::User, store::Store};
use thiserror::Error;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone)]
pub struct Claims {
    sub: String,
    exp: i64,
}

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Invalid token")]
    InvalidToken,
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("User not found")]
    UserNotFound,
    #[error("Internal server error")]
    InternalServerError,
    #[error("Unauthorized")]
    Unauthorized,
}

pub fn jwt_secret() -> String {
    dotenvy::var("JWT_SECRET").expect("JWT_SECRET must be set")
}

pub fn create_jwt(user: &User) -> Result<String, AuthError> {
    let claims = Claims {
        sub: user.id.to_string(),
        exp: (Utc::now() + Duration::days(7)).timestamp(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(&jwt_secret().as_bytes()),
    )
    .map_err(|_| AuthError::InternalServerError)?;
    Ok(token)
}

pub fn verify_jwt(token: &str) -> Result<User, AuthError> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret().as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| AuthError::InvalidToken)?;

    let user_id: Uuid = token_data.claims.sub.parse::<Uuid>().unwrap();
    let mut store = Store::new().map_err(|e| AuthError::DatabaseError(e.to_string()))?;
    let user = store
        .get_user_by_id(user_id)
        .map_err(|_| AuthError::InternalServerError)?
        .ok_or(AuthError::Unauthorized)?;

    Ok(user)
}
