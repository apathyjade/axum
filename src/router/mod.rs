mod user;
mod common;

use axum::Router;

use crate::AppStateArc;
pub fn all_routes() -> Router<AppStateArc> {
    Router::new()
        .nest("/user", user::router())
        .merge(common::router())
}