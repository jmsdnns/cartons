# MiniORM

Uses the [quote](https://docs.rs/quote/latest/quote/) and [syn](https://docs.rs/quote/latest/syn/) crates for implementing procedural macros. It also uses [SQLx](https://docs.rs/sqlx/latest/sqlx/) w/ SQLite for the database layer.

## CLI

Instead of `cargo run`, like the other cartons, each project in this carton uses `cargo test`.

```shell
$ cd micromacros/miniorm
$ cargo test
...
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

I recommend reading [the tests](tests/the_test.rs) before the macro, as it sets expectations nicely for what the macro is actually doing.


## Design

The goal of this project is to design an ORM the way Rust programmers would expect one to be designed, which means using procedural macros.

The basic idea is to analyze the struct and then generate 4 functions that mirror the usual CRUD operations: `select`, `insert`, `update`, and `delete`. To do this, a struct would need to use a macro to _derive_ `MiniORM` from their struct. We'll also derive `sqlx::FromRow`, as we're using SQLx to read row data directly into struct instances.

```rust
#[derive(MiniORM, sqlx::FromRow)]
pub struct Book {
    id: i32,
    title: String,
    pages: i32,
    author: String,
}
```

When we put `#[derive(MiniORM)]` on top, the macro will read the struct and generate the following functions using the struct's unique name, `Book`, and fields, `id`,`title`,`pages`,`author`.

```rust
impl Book {
    pub fn fields() -> Vec<&str> {
        ..
    }

    pub async fn select(pool: &SqlitePool, id: i32) -> Result<Book, Error> {
        ..
    }

    pub async fn insert(&self, pool: &SqlitePool) -> Result<SqliteQueryResult, Error> {
        ..
    }

    pub async fn update(&self, pool: &SqlitePool) -> Result<SqliteQueryResult, Error> {
        ..
    }

    pub async fn delete(pool: &SqlitePool, id: i32) -> Result<SqliteQueryResult, Error> {
        ..
    }
}
```

This allows us to define our struct, create an instance, and automatically have what we need to save it in a database.

```rust
let pool = db_connection_pool().await;

let book = Book {
    id: 1728,
    title: "My Story".to_string(),
    pages: 1337,
    author: "Jms Dnns".to_string(),
};

book.insert(&pool).await;
```

