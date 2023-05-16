use rocket_db_pools::{
    sqlx::{Row}
};


use rocket::{get, http::Status, response::Redirect, uri};


use crate::db::{ BlogDBC };
use crate::types::{Article, ArticleData, RtData};
use crate::article::db_service::get_article;

#[get("/")]
pub fn index() {
    Redirect::to(uri!("/get_article?all=true&id=-1&author_id=-1"));
}

/// get article
#[get("/get_article?<all>&<id>&<author_id>")]
pub async fn route_article(
    db: BlogDBC,
    all: bool,
    id: &str,
    author_id: &str,
) -> Result<RtData<ArticleData>, Status> {
    let result = get_article(db, all, id, author_id).await;

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

            Ok(RtData {
                success: true,
                msg: String::from("get all article success!"),
                rt: 1,
                data: ArticleData(articles.collect()),
            })
        }
        Err(err) => {
            println!("query all article error, {}", err);
            Err(Status::InternalServerError)
        }
    }
}
