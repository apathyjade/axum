
use std::env;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

// pub type PgPool = Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;
// pub type PgPoolConn = PooledConnection<AsyncDieselConnectionManager<AsyncPgConnection>>;

pub type DbPool = Pool<Postgres>;

pub async fn establish_connection_pool() -> DbPool {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL 必须在.env文件或环境变量中设置");

    PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_url).await.expect("数据库连接失败")
}