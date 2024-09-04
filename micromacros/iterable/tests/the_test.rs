use iterable::Iterable;
use iterable_trait::Iterable;
use std::any::TypeId;

#[derive(Iterable)]
pub struct Book {
    id: u64,
    title: String,
    pages: u64,
    author: String,
}

#[test]
fn check_field_types() {
    let b = Book {
        id: 1728,
        title: "My Story".to_string(),
        pages: 121,
        author: "Jms Dnns".to_string(),
    };

    let mut bi = b.iter();

    if let Some((_, value)) = bi.next() {
        assert_eq!(value.type_id(), TypeId::of::<u64>());
    }
    if let Some((_, value)) = bi.next() {
        assert_eq!(value.type_id(), TypeId::of::<String>());
    }
    if let Some((_, value)) = bi.next() {
        assert_eq!(value.type_id(), TypeId::of::<u64>());
    }
    if let Some((_, value)) = bi.next() {
        assert_eq!(value.type_id(), TypeId::of::<String>());
    }
}

#[test]
fn check_field_names_and_values() {
    let b = Book {
        id: 1728,
        title: "My Story".to_string(),
        pages: 121,
        author: "Jms Dnns".to_string(),
    };

    let mut bi = b.iter();

    if let Some((name, value)) = bi.next() {
        assert_eq!(name, "id");
        assert_eq!(value.downcast_ref::<u64>().unwrap(), &1728);
    }
    if let Some((name, value)) = bi.next() {
        assert_eq!(name, "title");
        assert_eq!(
            value.downcast_ref::<String>().unwrap(),
            &"My Story".to_string()
        );
    }
    if let Some((name, value)) = bi.next() {
        assert_eq!(name, "pages");
        assert_eq!(value.downcast_ref::<u64>().unwrap(), &121);
    }
    if let Some((name, value)) = bi.next() {
        assert_eq!(name, "author");
        assert_eq!(
            value.downcast_ref::<String>().unwrap(),
            &"Jms Dnns".to_string()
        );
    }
}
