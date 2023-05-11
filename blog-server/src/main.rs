#[macro_use]
extern crate rocket;
use rocket_db_pools::{
    sqlx::{self, postgres::PgRow, Row},
    Connection, Database,
};

use rocket::{
    http::Status,
    serde::json::{self, serde_json::json, Value},
};

mod data_types;
mod db_service;
use data_types::{AllArticleData, Article, FailureData};
use db_service::{get_article};

#[derive(Database)]
#[database("blog")]
struct Blog(sqlx::PgPool);

/// get article
#[get("/get_article?<all>&<id>&<user_id>")]
async fn index(mut db: Connection<Blog>, all: bool, id: &str, user_id: &str) -> Result<Value, Status> {
    let sql: String = get_article(all, id, user_id);
    
    let result: Result<Vec<PgRow>, sqlx::Error> = sqlx::query(&sql).fetch_all(&mut *db).await;

    match result {
        Ok(v) => {
            let articles = v.iter().map(|row| Article {
                id: row.get(0),
                title: row.get(1),
                content: row.get(2),
                author_id: row.get(3),
                author_name: row.get(4),
                author_desc: row.get(5),
            });

            Ok(json!({
                "success": true,
                "msg": String::from("get all article success!"),
                "data": AllArticleData(articles.collect()),
            }))
        }
        Err(err) => {
            println!("query all article error, {}", err);
            Err(Status::InternalServerError)
        }
    }
}

// #[get("/get_user?<id>")]
// fn get_user_by_id (mut db: Connection<Blog>, id: &str) -> Result<Value, Status> {
//     let sql = sql_user_by_id(id);
// }

#[catch(500)]
fn error_catcher() -> Option<Value> {
    Some(json!({
            "success": false,
            "msg": String::from("get all article fail!"),
            "data": FailureData(()),
    }))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Blog::init())
        .register("/", catchers![error_catcher])
        .mount("/", routes![index])
}
