use mongodb::bson::{doc, document::Document};
use crate::db::{MongoRepository, MongoEntity};
use crate::models::books::{Book};
use crate::routes::books::NewBook;

pub struct BooksRepository {}

impl MongoEntity for Book {
    fn collection() -> String {
        String::from("books")
    }
}

pub fn new_book_to_doc(new_book: NewBook) -> Document {
    doc! {
        "name": new_book.name,
        "author": new_book.author,
        "categories": new_book.categories
    }
}

impl MongoRepository for BooksRepository {
    type Entity = Book;
}
