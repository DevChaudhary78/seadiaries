// modules
mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;

// entry point imports
use api::blog_api::{create_blog, delete_blog, get_all_blogs, get_blog, update_blog};
use repository::mongodb_repo::MongoRepo;

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
        .manage(db)
        .mount(
            "/",
            routes![
                create_blog,
                get_blog,
                update_blog,
                delete_blog,
                get_all_blogs
            ],
        )
        .mount(
            "/users",
            routes![
                create_blog,
                get_all_blogs,
                update_blog,
                delete_blog,
                get_blog
            ],
        )
}
