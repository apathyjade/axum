
use std::env;

use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};


pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub async fn init_diesel_db() -> DbPool {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL 必须在.env文件或环境变量中设置");

    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder()
        .max_size(10)
        .build(manager)
        .expect("F数据库连接失败")
}

// use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
// pub type DbPool = Pool<Postgres>;
// pub async fn establish_connection_pool() -> DbPool {
//     let db_url = env::var("DATABASE_URL").expect("DATABASE_URL 必须在.env文件或环境变量中设置");

//     PgPoolOptions::new()
//         .max_connections(10)
//         .connect(&db_url).await.expect("数据库连接失败")
// }

