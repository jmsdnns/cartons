//#![allow(unused)]

use anyhow::Result;
use axum::{routing::get, Router};
use sqlx::{Pool, Postgres};

mod auth;
mod errors;
mod models;
mod routes;

use crate::routes::account;
use crate::routes::landing;

#[derive(Clone, Debug)]
pub struct AppState {
    db: Pool<Postgres>,
    hmac_key: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let username = "postgres";
    let password = "postgres";
    let db_host = "0.0.0.0";
    let db_name = "mydb";
    let hmac_key = "my secret key";

    let db = models::init_db_pool(username, password, db_host, db_name).await?;
    sqlx::migrate!().run(&db).await?;

    let state = AppState {
        hmac_key: String::from(hmac_key),
        db,
    };

    let app = Router::new()
        .route("/landing", get(landing::home))
        .nest("/account", account::router(state.clone()))
        .with_state(state);

    // run our app with hyper, listening globally on port 5454
    let listener = tokio::net::TcpListener::bind("localhost:5454")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
