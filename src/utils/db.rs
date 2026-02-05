use diesel::{
    PgConnection,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};

use crate::utils;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConn = PooledConnection<ConnectionManager<PgConnection>>;
pub async fn init_diesel_db() -> DbPool {
    let db_url = utils::env::get_env(utils::env::Env::DatabaseUri);

    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder()
        .max_size(10)
        .build(manager)
        .expect("F数据库连接失败")
}

pub async fn init_gis_db() -> DbPool {
    let db_url = utils::env::get_env(utils::env::Env::GisDatabaseUri);

    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder()
        .max_size(10)
        .build(manager)
        .expect("F数据库连接失败")
}
