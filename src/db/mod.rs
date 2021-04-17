use mongodb::bson::{doc, document::Document, oid::ObjectId};

use crate::config::DB;
use crate::errors::Error;

use crate::errors::Error::{MongoQueryError, InvalidIDError};


pub mod books;

static DB_NAME: &'static str = "books";

pub trait MongoEntity {
    fn collection() -> String;
}

pub trait MongoRepository {
    type Entity: MongoEntity;

    fn find_by_id(id: &str, db: &DB) -> Option<Document> {
        let oid = ObjectId::with_string(id).expect("Failed to create ObjectId");

        let query = doc! {
            "_id": oid
        };

        match db.client.database(DB_NAME)
            .collection_with_type::<Document>(Self::Entity::collection().as_str()).find_one(query, None) {
            Ok(document) => document,
            Err(_) => None
        }
    }

    fn find_all(db: &DB) -> Result<Vec<Document>, Error> {
        let mut cursor = db.client.database(DB_NAME).collection_with_type::<Document>(Self::Entity::collection().as_str())
            .find(None, None)
            .map_err(MongoQueryError)?;

        let mut result: Vec<Document> = Vec::new();
        while let Some(doc) = cursor.next() {
            result.push(doc?);
        }
        Ok(result)
    }

    fn update(document: Document, db: &DB) -> Result<(), Error> {
        let id = document.get_str("_id")?;
        let oid = ObjectId::with_string(id)
            .map_err(|_| InvalidIDError(id.to_owned()))?;
        let query = doc! {
            "_id": oid,
        };

        db.client.database(DB_NAME).collection(Self::Entity::collection().as_str())
            .update_one(query, document, None)?;
        Ok(())
    }

    fn save(document: Document, db: &DB) -> Result<(), Error> {
        db.client.database(DB_NAME).collection(Self::Entity::collection().as_str())
            .insert_one(document, None)
            .map_err(MongoQueryError)?;
        Ok(())
    }
}