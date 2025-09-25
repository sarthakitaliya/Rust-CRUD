use crate::util::create_jwt;
use axum::{Json, http::StatusCode};
use bcrypt::DEFAULT_COST;
use serde::{Deserialize, Serialize};
use store::{
    models::user::{NewUser, User},
    store::Store,
};
use tokio::task;

use crate::error::ApiError;

#[derive(Serialize, Deserialize)]
pub struct AuthPayload {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: User,
}

pub async fn register(Json(payload): Json<AuthPayload>) -> Result<Json<User>, ApiError> {
    if payload.email.is_empty() || payload.password.is_empty() {
        let error = ApiError {
            status: StatusCode::BAD_REQUEST,
            message: "Email and password are required".to_string(),
        };
        return Err(error);
    }

    let new_user = task::spawn_blocking(move || -> Result<User, ApiError> {
        let mut store = Store::new().map_err(|e| ApiError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: format!("Db connection error: {}", e),
        })?;
        let found = store
            .get_user_by_email(&payload.email)
            .map_err(|e| ApiError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: format!("Db error: {}", e),
            })?;
        if found.is_some() {
            return Err(ApiError {
                status: StatusCode::BAD_REQUEST,
                message: "User already exists".to_string(),
            });
        }
        let hashed_password =
            bcrypt::hash(&payload.password, DEFAULT_COST).map_err(|e| ApiError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: format!("Hashing error: {}", e),
            })?;

        let created_user = NewUser {
            email: payload.email,
            password: hashed_password,
        };
        let user = store.create_user(created_user).unwrap();
        Ok(user)
    })
    .await
    .map_err(|e| ApiError {
        status: StatusCode::INTERNAL_SERVER_ERROR,
        message: format!("spawn blocking join error: {}", e),
    })??;

    Ok(Json(new_user))
}

pub async fn login(Json(payload): Json<AuthPayload>) -> Result<Json<LoginResponse>, ApiError> {
    if payload.email.is_empty() || payload.password.is_empty() {
        let error = ApiError {
            status: StatusCode::BAD_REQUEST,
            message: "Email and password are required".to_string(),
        };
        return Err(error);
    }

    let res = task::spawn_blocking(move || -> Result<LoginResponse, ApiError> {
        let mut store = Store::new().map_err(|e| ApiError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: format!("Db connection error: {}", e),
        })?;
        let found_opt = store
            .get_user_by_email(&payload.email)
            .map_err(|e| ApiError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: format!("Db error: {}", e),
            })?;
        let user = found_opt.ok_or(ApiError {
            status: StatusCode::NOT_FOUND,
            message: "User not found".to_string(),
        })?;
        let ok = bcrypt::verify(&payload.password, &user.password).map_err(|e| ApiError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: format!("bcrypt verify error: {}", e),
        })?;

        if !ok {
            return Err(ApiError {
                status: StatusCode::UNAUTHORIZED,
                message: "Invalid credentials".to_string(),
            });
        }
        let token = create_jwt(&user).map_err(|e| ApiError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: format!("Token error: {}", e),
        })?;

        Ok(LoginResponse { token, user })
    })
    .await
    .map_err(|e| ApiError {
        status: StatusCode::INTERNAL_SERVER_ERROR,
        message: format!("spawn blocking join error: {}", e),
    })??;

    Ok(Json(res))
}
