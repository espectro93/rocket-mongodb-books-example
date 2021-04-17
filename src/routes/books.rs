use rocket_contrib::json::{Json, JsonValue};
use mongodb::bson::{doc};
use serde::{Deserialize};

use crate::config::DB;
use crate::db::books::{BooksRepository, new_book_to_doc};
use crate::db::{MongoRepository};
use std::error::Error;

#[get("/books")]
pub fn get_books(db: DB) -> Result<JsonValue, Box<dyn Error>> {
    let result = BooksRepository::find_all(&db)?;
    Ok(json!({ "books": result }))
}

#[get("/books/<id>")]
pub fn get_book(id: String, db: DB) -> Option<JsonValue> {
    let result = BooksRepository::find_by_id(id.as_str(), &db)?;
    Some(json!(result))
}

#[derive(Deserialize)]
pub struct NewBook {
    pub name: String,
    pub author: String,
    pub categories: Vec<String>,
}

#[post("/books", format = "json", data = "<new_book>")]
pub fn create_book(new_book: Json<NewBook>, db: DB) -> Result<(), Box<dyn Error>>{
    BooksRepository::save(new_book_to_doc(NewBook {
        name: new_book.name.to_string(),
        author: new_book.author.to_string(),
        categories: new_book.categories.to_vec(),
    }), &db)?;
    Ok(())
}