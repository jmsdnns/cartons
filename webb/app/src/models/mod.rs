pub mod user;

use anyhow::Result;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub async fn init_db_pool(
    username: &str,
    password: &str,
    host: &str,
    dbname: &str,
) -> Result<Pool<Postgres>> {
    let db_url = format!(
        "postgres://{}:{}@{}:5432/{}",
        username, password, host, dbname
    );

    println!("DBURL: {}", db_url);

    let db = PgPoolOptions::new()
        .max_connections(20)
        .connect(&db_url)
        .await?;

    Ok(db)
}
