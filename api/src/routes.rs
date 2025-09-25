use crate::handlers;
use axum::{
    Router,
    routing::{get, post},
};

pub fn router() -> Router {
    Router::new()
        .route("/api/login", post(handlers::auth::login))
        .route("/api/register", post(handlers::auth::register))
        .route(
            "/api/bookmark",
            get(handlers::bookmark::get_bookmarks)
                .post(handlers::bookmark::create_bookmark)
                .patch(handlers::bookmark::set_favorite),
        )
        .route(
            "/api/bookmark/{bookmark_id}",
            get(handlers::bookmark::get_bookmark).delete(handlers::bookmark::delete_bookmark),
        )
}
