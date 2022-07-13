use crate::{models::blog_model::Blog, repository::mongodb_repo::MongoRepo};
use mongodb::{bson::oid::ObjectId, results::InsertOneResult};
use rocket::{http::Status, serde::json::Json, State};

#[post("/blog", data = "<new_blog>")]
pub fn create_blog(
    db: &State<MongoRepo>,
    new_blog: Json<Blog>,
) -> Result<Json<InsertOneResult>, Status> {
    let data = Blog {
        id: None,
        title: new_blog.title.to_owned(),
        body: new_blog.body.to_owned(),
        author: new_blog.author.to_owned(),
        likes: new_blog.likes.to_owned(),
        dislikes: new_blog.dislikes.to_owned(),
    };
    let user_detail = db.create_blog(data);
    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/blog/<path>")]
pub fn get_blog(db: &State<MongoRepo>, path: String) -> Result<Json<Blog>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };

    let blog_details = db.get_blog(&id);

    match blog_details {
        Ok(blog) => Ok(Json(blog)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/blog/<path>", data = "<new_blog>")]
pub fn update_blog(
    db: &State<MongoRepo>,
    path: String,
    new_blog: Json<Blog>,
) -> Result<Json<Blog>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    }

    let data = Blog {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        title: new_blog.title.to_owned(),
        body: new_blog.body.to_owned(),
        author: new_blog.body.to_owned(),
        likes: new_blog.likes.to_owned(),
        dislikes: new_blog.dislikes.to_owned(),
    };

    let update_result = db.update_blog(&id, data);

    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_blog_info = db.get_blog(&id);

                return match updated_blog_info {
                    Ok(blog) => Ok(Json(blog)),
                    Err(_) => Err(Status::InternalServerError),
                };
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/blog/<path>")]
pub fn delete_blog(db: &State<MongoRepo>, path: String) -> Result<Json<&str>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };

    let delete_result = db.delete_blog(&id);

    match delete_result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return Ok(Json("Blog successfullly deleted!"));
            } else {
                Err(Status::NotFound)
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/blogs")]
pub fn get_all_blogs(db: &State<MongoRepo>) -> Result<Json<Vec<Blog>>, Status> {
    let blogs = db.get_all_blogs();

    match blogs {
        Ok(blogs) => Ok(Json(blogs)),
        Err(_) => Err(Status::InternalServerError),
    }
}
