#![allow(dead_code)]
use dbmodel::DBModel;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use std::fs;

#[derive(DBModel, sqlx::FromRow)]
pub struct Book {
    id: i32,
    title: String,
    pages: i32,
    author: String,
}

async fn mkdb(dbname: &str) -> SqlitePool {
    let options = SqliteConnectOptions::new()
        .filename(dbname)
        .create_if_missing(true);

    let pool = sqlx::sqlite::SqlitePool::connect_with(options)
        .await
        .unwrap();

    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS book
        (
            id INTEGER PRIMARY KEY NOT NULL,
            title TEXT NOT NULL,
            pages INTEGER NOT NULL,
            author TEXT NOT NULL
        );"#,
    )
    .execute(&pool)
    .await
    .unwrap();

    pool
}

fn rmdb(dbname: &str) {
    fs::remove_file(dbname).unwrap();
}

async fn addbook(pool: &SqlitePool, id: i32) {
    sqlx::query(r#"INSERT INTO book VALUES ($1, "Meow", 10, "Sierra")"#)
        .bind(id)
        .execute(pool)
        .await
        .unwrap();
}

#[tokio::test]
async fn gen_insert() {
    let pool = mkdb("gen_insert").await;

    let book = Book {
        id: 1728,
        title: "My Story".to_string(),
        pages: 1337,
        author: "Jms Dnns".to_string(),
    };

    book.insert(&pool).await;

    rmdb("gen_insert");
}

#[tokio::test]
async fn gen_select() {
    let pool = mkdb("gen_select").await;
    addbook(&pool, 42).await;
    let book = Book::select(&pool, 42);
    rmdb("gen_select");
}

#[test]
fn gen_fields() {
    let field_list = Book::fields();
    assert_eq!(vec!["id", "title", "pages", "author"], field_list)
}
