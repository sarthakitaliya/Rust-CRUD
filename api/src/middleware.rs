use axum::{
    http::{StatusCode},
};
use store::models::user::User;

use crate::util::{ verify_jwt};

pub async fn auth_midd(token: &str) -> Result<User, StatusCode>{
    let token_data = verify_jwt(token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    Ok(token_data)
}
