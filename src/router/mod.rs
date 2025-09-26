mod user;
mod common;
use axum::{ Router, middleware };
use crate::middleware as custom_middleware;

use crate::AppStateArc;
pub fn all_routes() -> Router<AppStateArc> {
    Router::new()
        .nest("/user", user::router())
        .merge(common::router())
        .layer(middleware::from_fn(custom_middleware::error::deal_error))
}