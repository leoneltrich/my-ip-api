use axum::{
    Router,
    routing::get,
    middleware,
};
use crate::services::auth_service::auth_middleware;
use crate::services::ip_service::get_current_ip;

pub fn create_router() -> Router {
    Router::new()
        .route("/ip", get(get_current_ip))
        .layer(middleware::from_fn(auth_middleware))
}