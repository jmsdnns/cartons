#![allow(dead_code)]
use micromacros::DBModel;

#[derive(DBModel)]
pub struct Book {
    id: u64,
    title: String,
    pages: u64,
    author: String,
}

#[test]
fn gen_select() {
    let select_sql = Book::select();
    assert_eq!("select id,title,pages,author from book;", select_sql);
}

#[test]
fn gen_insert() {
    let book = Book {
        id: 1728,
        title: "My Story".to_string(),
        pages: 1337,
        author: "Jms Dnns".to_string(),
    };
    let insert_sql = book.insert();
    assert_eq!(
        "insert into book (id,title,pages,author) values(?,?,?,?);",
        insert_sql
    );
}
