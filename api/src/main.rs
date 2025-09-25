use dotenvy::dotenv;

use crate::routes::router;
pub mod error;
pub mod handlers;
pub mod middleware;
pub mod routes;
pub mod util;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = router();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
