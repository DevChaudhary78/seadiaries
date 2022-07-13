use std::env;
extern crate dotenv;
use dotenv::dotenv;

use crate::models::blog_model::Blog;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    sync::{Client, Collection},
};

pub struct MongoRepo {
    col: Collection<Blog>,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };

        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rustDB");
        let col: Collection<Blog> = db.collection("Blog");
        MongoRepo { col }
    }

    pub fn create_blog(&self, new_blog: Blog) -> Result<InsertOneResult, Error> {
        let new_doc = Blog {
            id: None,
            title: new_blog.title,
            body: new_blog.body,
            author: new_blog.author,
            likes: new_blog.likes,
            dislikes: new_blog.dislikes,
        };

        let blog = self
            .col
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating blog");
        Ok(blog)
    }

    pub fn get_blog(&self, id: &String) -> Result<Blog, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let blog_details = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("Error getting blog details");

        Ok(blog_details.unwrap())
    }

    pub fn update_blog(&self, id: &String, new_blog: Blog) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};

        let new_doc = doc! {
            "$set": {
                "id": new_blog.id,
                "title": new_blog.title,
                "body": new_blog.body,
                "author": new_blog.author,
                "likes": new_blog.likes,
                "dislikes": new_blog.dislikes
            },
        };

        let updated_doc = self
            .col
            .update_one(filter, new_doc, None)
            .ok()
            .expect("Error updating blog");

        Ok(updated_doc)
    }

    pub fn delete_blog(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};

        let blog_details = self
            .col
            .delete_one(filter, None)
            .ok()
            .expect("Error deleting blog");

        Ok(blog_details)
    }

    pub fn get_all_blogs(&self) -> Result<Vec<Blog>, Error> {
        let cursors = self
            .col
            .find(None, None)
            .ok()
            .expect("Error getting lists of blogs");
        let blogs = cursors.map(|doc| doc.unwrap()).collect();
        Ok(blogs)
    }
}
