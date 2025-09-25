use crate::schema::bookmark;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Queryable, Serialize)]
pub struct Bookmark {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub title: String,
    pub url: String,
    pub description: Option<String>,
    pub is_favorite: Option<bool>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = bookmark)]
pub struct NewBookmark {
    pub user_id: Uuid,
    pub title: String,
    pub url: String,
    pub description: Option<String>,
    pub is_favorite: Option<bool>,
}
