use crate::utils::db;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db_pool: db::DbPool,
}

pub type AppStateArc = Arc<AppState>;