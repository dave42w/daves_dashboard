use axum::{
    routing::{get, post},
    Router,
};
use crate::controllers::sys_info::{get_sys_info, post_sys_info};

pub fn create_routes() -> Router {
    Router::new()
    .route("/sys_info", get(get_sys_info))
    .route("/sys_info", post(post_sys_info))
}