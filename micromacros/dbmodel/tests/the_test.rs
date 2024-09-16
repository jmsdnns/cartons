#![allow(dead_code)]
use dbmodel::DBModel;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use std::fs;

#[derive(DBModel, sqlx::FromRow, Debug)]
pub struct Book {
    id: i32,
    title: String,
    pages: i32,
    author: String,
}

#[tokio::test]
async fn gen_select() {
    // setup
    let pool = mkdb("gen_select").await;
    addbook(&pool, 42, "Meow", 10, "Sierra").await;

    let b = Book::select(&pool, 42).await;
    assert_eq!(b.id, 42);
    assert_eq!(b.title, "Meow".to_string());
    assert_eq!(b.pages, 10);
    assert_eq!(b.author, "Sierra".to_string());

    // teardown
    rmdb("gen_select");
}

#[tokio::test]
async fn gen_insert() {
    // setup
    let pool = mkdb("gen_insert").await;

    let book = Book {
        id: 1728,
        title: "My Story".to_string(),
        pages: 1337,
        author: "Jms Dnns".to_string(),
    };
    book.insert(&pool).await;

    let b = loadbook(&pool, 1728).await.unwrap();
    assert_eq!(b.id, book.id);
    assert_eq!(b.title, book.title);
    assert_eq!(b.pages, book.pages);
    assert_eq!(b.author, book.author);

    rmdb("gen_insert");
}

#[tokio::test]
async fn gen_delete() {
    // setup
    let pool = mkdb("gen_delete").await;
    addbook(&pool, 42, "Meow", 10, "Sierra").await;

    Book::delete(&pool, 42).await;
    loadbook(&pool, 42).await.unwrap_err();

    // teardown
    rmdb("gen_delete");
}

#[test]
fn gen_fields() {
    let field_list = Book::fields();
    assert_eq!(vec!["id", "title", "pages", "author"], field_list)
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

async fn addbook(pool: &SqlitePool, id: i32, title: &str, pages: i32, author: &str) {
    sqlx::query(r#"INSERT INTO book VALUES ($1, $2, $3, $4)"#)
        .bind(id)
        .bind(title)
        .bind(pages)
        .bind(author)
        .execute(pool)
        .await
        .unwrap();
}

async fn loadbook(pool: &SqlitePool, id: i32) -> Result<Book, sqlx::error::Error> {
    sqlx::query_as(r#"SELECT * FROM book WHERE id = $1"#)
        .bind(id)
        .fetch_one(pool)
        .await
}
