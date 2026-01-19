pub mod auth;
pub mod common;
pub mod tenant;
pub mod user;

use crate::middleware as custom_middleware;
use axum::{Router, middleware};

use crate::AppStateArc;
pub fn all_routes() -> Router<AppStateArc> {
    Router::new()
        .nest(
            "/api",
            Router::new()
                .nest("/user", user::router())
                .nest("/tenant", tenant::router())
                .nest("/auth", auth::router()),
        )
        .merge(common::router())
        .layer(middleware::from_fn(custom_middleware::error::deal_error))
}
