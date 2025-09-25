use axum::{extract::Path, http::StatusCode, Json};
use axum_extra::{
    extract::TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use serde::Deserialize;
use store::{
    models::bookmark::{Bookmark, NewBookmark},
    store::Store,
};
use tokio::task;
use uuid::Uuid;

use crate::{error::ApiError, middleware::auth_midd};

#[derive(Deserialize)]
pub struct NewBInput {
    pub title: String,
    pub url: String,
    pub description: Option<String>,
    pub is_favorite: Option<bool>,
}

#[derive(Deserialize)]
pub struct FavInput {
    pub bookmark_id: Uuid,
    pub is_favorite: bool,
}

pub async fn create_bookmark(
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    Json(payload): Json<NewBInput>,
) -> Result<Json<Bookmark>, ApiError> {
    let user = auth_midd(bearer.token()).await.map_err(|_e| ApiError {
        status: StatusCode::INTERNAL_SERVER_ERROR,
        message: "Something went wrong".into(),
    })?;
    let user_id: Uuid = user.id;
    let new_bookmark = NewBookmark {
        user_id: user_id,
        title: payload.title,
        url: payload.url,
        description: payload.description,
        is_favorite: payload.is_favorite,
    };

    let created = task::spawn_blocking(move || -> Result<Bookmark, ApiError> {
        let mut store = Store::new().map_err(|e| ApiError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: format!("Db error: {}", e),
        })?;

        let bookmark = store.create_bookmark(new_bookmark).map_err(|e| ApiError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: format!("insert error: {}", e),
        })?;
        Ok(bookmark)
    })
    .await
    .map_err(|e| ApiError {
        status: StatusCode::INTERNAL_SERVER_ERROR,
        message: format!("spawn blocking join error: {}", e),
    })??;
    Ok(Json(created))
}

pub async fn get_bookmarks(
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
) -> Result<Json<Vec<Bookmark>>, ApiError>{
    let user = auth_midd(bearer.token()).await.map_err(|_e| ApiError {
        status: StatusCode::INTERNAL_SERVER_ERROR,
        message: "Something went wrong".into(),
    })?;
    let user_id: Uuid = user.id;

    let bookmarks = task::spawn_blocking(move || -> Result<Vec<Bookmark>, ApiError>{
        let mut store = Store::new().map_err(|e| ApiError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: format!("Db error: {}", e),
        })?;

        let bookmarks = store.list_bookmarks(user_id, 10).map_err(|e| ApiError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: format!("List error: {}", e),
        })?;
        Ok(bookmarks)
    })
    .await
    .map_err(|e| ApiError {
        status: StatusCode::INTERNAL_SERVER_ERROR,
        message: format!("spawn blocking join error: {}", e),
    })??;
    Ok(Json(bookmarks))
}

pub async fn get_bookmark(
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    Path(bookmark_id): Path<Uuid>,
) -> Result<Json<Bookmark>, ApiError> {
    let _user = auth_midd(bearer.token()).await.map_err(|_e| ApiError {
        status: StatusCode::INTERNAL_SERVER_ERROR,
        message: "Something went wrong".into(),
    })?;
    let res = task::spawn_blocking(move || -> Result<Bookmark, ApiError> {
        let mut store = Store::new().map_err(|e| ApiError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: format!("Db error: {}", e),
        })?;
        let bookmark = store.get_bookmark(bookmark_id).map_err(|e| ApiError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: format!("Get error: {}", e),
        })?;
        Ok(bookmark)
    })
    .await
    .map_err(|e| ApiError {
        status: StatusCode::INTERNAL_SERVER_ERROR,  
        message: format!("spawn blocking join error: {}", e),
    })??;
    Ok(Json(res))
}    

pub async fn delete_bookmark(
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    Path(bookmark_id): Path<Uuid>,
) -> Result<Json<Bookmark>, ApiError> {
    let _user = auth_midd(bearer.token()).await.map_err(|_e| ApiError {
        status: StatusCode::INTERNAL_SERVER_ERROR,
        message: "Something went wrong".into(),
    })?;
    let res = task::spawn_blocking(move || -> Result<Bookmark, ApiError> {
        let mut store = Store::new().map_err(|e| ApiError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: format!("Db error: {}", e),
        })?;
        let deleted = store.delete_bookmark(bookmark_id).map_err(|e| ApiError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: format!("Delete error: {}", e),
        })?;
        Ok(deleted)
    })
    .await
    .map_err(|e| ApiError {
        status: StatusCode::INTERNAL_SERVER_ERROR,
        message: format!("spawn blocking join error: {}", e),
    })??;
    Ok(Json(res))
}

pub async fn set_favorite(
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    Json(payload): Json<FavInput>,
) -> Result<Json<Bookmark>, ApiError> {
    let _user = auth_midd(bearer.token()).await.map_err(|_e| ApiError {
        status: StatusCode::INTERNAL_SERVER_ERROR,
        message: "Something went wrong".into(),
    })?;
    let res = task::spawn_blocking(move || -> Result<Bookmark, ApiError> {
        let mut store = Store::new().map_err(|e| ApiError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: format!("Db error: {}", e),
        })?;
        let updated = store.set_fav(payload.bookmark_id, payload.is_favorite).map_err(|e| ApiError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: format!("Update error: {}", e),
        })?;    
        Ok(updated)
    })
    .await
    .map_err(|e| ApiError {
        status: StatusCode::INTERNAL_SERVER_ERROR,
        message: format!("spawn blocking join error: {}", e),
    })??;
    Ok(Json(res))
}