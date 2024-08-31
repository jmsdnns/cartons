//#![allow(dead_code)]
// #![allow(unused_variables, unused_imports)]

use anyhow::{anyhow, Context};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres, Row};

const INSERT_USER: &str = "INSERT INTO users (username, password_hash) \
    values($1, $2) \
    RETURNING id";

const SELECT_USER: &str = "SELECT id, username, password_hash \
    FROM users \
    WHERE username = $1";

const UPDATE_USER_PASS: &str = "UPDATE users \
    SET password_hash = $2 \
    WHERE username = $1";

pub async fn init_db_pool(
    username: &str,
    password: &str,
    host: &str,
    dbname: &str,
) -> Result<Pool<Postgres>, sqlx::Error> {
    let db_url = format!(
        "postgres://{}:{}@{}:5432/{}",
        username, password, host, dbname
    );

    let db = PgPoolOptions::new()
        .max_connections(20)
        .connect(&db_url)
        .await?;

    Ok(db)
}

#[derive(sqlx::FromRow, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
}

impl User {
    pub async fn create(
        db: &Pool<Postgres>,
        username: String,
        password: String,
    ) -> anyhow::Result<i32> {
        let result = sqlx::query(INSERT_USER)
            .bind(&username)
            .bind(&password)
            .fetch_one(db)
            .await
            .map_err(|e| match e {
                sqlx::Error::Database(dbe) if dbe.constraint() == Some("users_username_key") => {
                    anyhow!(dbe).context("username taken")
                }
                _ => anyhow!(e),
            })?;

        Ok(result.get("id"))
    }

    pub async fn load(db: &Pool<Postgres>, username: String) -> anyhow::Result<User> {
        let result: User = sqlx::query_as(SELECT_USER)
            .bind(&username)
            .fetch_one(db)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => anyhow!(e).context("user not found"),
                _ => anyhow!(e), //.context(e.to_string()),
            })?;

        Ok(result)
    }

    pub async fn update_pass(
        db: &Pool<Postgres>,
        username: String,
        password: String,
    ) -> anyhow::Result<()> {
        sqlx::query(UPDATE_USER_PASS)
            .bind(&username)
            .bind(&password)
            .execute(db)
            .await
            .map_err(|e| match e {
                sqlx::Error::Database(dbe) => anyhow!(dbe),
                _ => anyhow!(e).context("unknown update error"),
            })?;

        Ok(())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Get DB details from environment
    let username = dotenvy::var("POSTGRES_USER").context("POSTGRES_USER not set")?;
    let password = dotenvy::var("POSTGRES_PASSWORD").context("POSTGRES_PASSWORD not set")?;
    let host = "0.0.0.0";
    let dbname = dotenvy::var("POSTGRES_DB").context("POSTGRES_DB not set")?;

    let db = init_db_pool(&username, &password, host, &dbname).await?;
    sqlx::migrate!().run(&db).await?;

    println!("Creating a user");
    let result = User::create(&db, String::from("jmsdnns"), String::from("foo")).await;
    match result {
        Ok(user_id) => println!("SUCCESS: {}", user_id),
        Err(e) => eprintln!("ERROR: {}", e),
    }

    println!("Loading the user");
    let result = User::load(&db, String::from("jmsdnns")).await;
    match result {
        Ok(user) => println!("SUCCESS: {:?}", user),
        Err(e) => eprintln!("ERROR: {}", e),
    }

    println!("Updating the password");
    match User::update_pass(&db, String::from("jmsdnns"), String::from("foofighters")).await {
        Ok(_) => println!("SUCCESS"),
        Err(e) => eprintln!("ERROR: {}", e),
    }

    Ok(())
}
