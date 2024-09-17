#![allow(dead_code)]
use miniorm::MiniORM;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use std::fs;

#[derive(MiniORM, sqlx::FromRow, Debug)]
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

    // Put book instance in database without ORM
    addbook(&pool, 42, "Meow", 10, "Sierra").await;

    // Load book instance with ORM and check values
    let b = Book::select(&pool, 42).await.unwrap();
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

    // Create book instance and save to DB with ORM
    let book = Book {
        id: 1728,
        title: "My Story".to_string(),
        pages: 1337,
        author: "Jms Dnns".to_string(),
    };
    let result = book.insert(&pool).await.unwrap();
    let count = result.rows_affected();
    assert_eq!(count, 1);

    // Load new instance of book without ORM and compare
    let b = loadbook(&pool, 1728).await.unwrap();
    assert_eq!(b.id, book.id);
    assert_eq!(b.title, book.title);
    assert_eq!(b.pages, book.pages);
    assert_eq!(b.author, book.author);

    // teardown
    rmdb("gen_insert");
}

#[tokio::test]
async fn gen_update() {
    // setup
    let pool = mkdb("gen_update").await;

    // Put book instance in database and load it without ORM
    addbook(&pool, 42, "Meow", 10, "Sierra").await;
    let mut book = loadbook(&pool, 42).await.unwrap();

    // Change a value and update it in the DB with ORM
    book.pages = 14;
    book.update(&pool).await.unwrap();

    // Load new instance of book without ORM and compare
    let b = loadbook(&pool, 42).await.unwrap();
    assert_eq!(b.pages, book.pages);

    // teardown
    rmdb("gen_update");
}

#[tokio::test]
async fn gen_delete() {
    // setup
    let pool = mkdb("gen_delete").await;

    // Put book instance in database without ORM
    addbook(&pool, 42, "Meow", 10, "Sierra").await;

    // Delete book from DB with ORM and verify single row is gone
    let result = Book::delete(&pool, 42).await.unwrap();
    let count = result.rows_affected();
    assert_eq!(count, 1);

    // Try loading book without ORM and verify error is thrown
    loadbook(&pool, 42).await.unwrap_err();

    // teardown
    rmdb("gen_delete");
}

#[test]
fn gen_fields() {
    let field_list = Book::fields();
    assert_eq!(vec!["id", "title", "pages", "author"], field_list)
}

//
// HELPERS
//

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
