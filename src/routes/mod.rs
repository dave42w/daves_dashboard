mod home;

use axum::{
    routing::{get, post},
    Router,
};
use home::{get_home, post_home};

pub fn create_routes() -> Router {
    Router::new()
    .route("/", get(get_home))
    .route("/", post(post_home))
}