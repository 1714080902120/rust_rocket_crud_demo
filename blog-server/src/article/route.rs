use rocket_db_pools::sqlx::Row;

use rocket::{get, post, http::Status, response::Redirect, uri};

use crate::article::db_service::{get_article, try_get_user_article};
use crate::article::UserArticleType;
use crate::db::BlogDBC;
use crate::types::{Article, ArticleData, RtData};

use super::{UserArticle, UserAticleParams, AddArticleData};

#[get("/")]
pub fn index() {
    Redirect::to(uri!("/get_article?all=true&id=-1&author_id=-1"));
}

/// get article
#[get("/get_article?<all>&<id>")]
pub async fn route_article(
    db: BlogDBC,
    all: bool,
    id: &str,
) -> Result<RtData<ArticleData>, Status> {
    let result = get_article(db, all, id).await;

    match result {
        Ok(v) => {
            let articles = v.iter().map(|row| Article {
                id: row.get(0),
                title: row.get(1),
                content: row.get(2),
                author_name: row.get(4),
                author_desc: row.get(5),
            });

            Ok(RtData {
                success: true,
                msg: String::from("get all article success!"),
                rt: 1,
                data: ArticleData {
                    list: articles.collect(),
                },
            })
        }
        Err(err) => {
            println!("query all article error, {}", err);
            Err(Status::InternalServerError)
        }
    }
}

#[get("/article", data = "<params>")]
pub async fn get_user_article<'r>(
    params: UserAticleParams,
    db: BlogDBC,
) -> Result<RtData<UserArticleType>, Status> {
    dbg!(&params);

    let result = try_get_user_article(
        db,
        params.all,
        params.user_id.as_str(),
        params.article_id.as_str(),
    )
    .await;

    match result {
        Ok(v) => {
            let data = v
                .iter()
                .map(|item| UserArticle {
                    id: item.get::<sqlx::types::Uuid, usize>(0).to_string(),
                    title: item.get(1),
                    content: item.get(2),
                })
                .collect();

            return Ok(RtData {
                success: true,
                rt: 1,
                msg: String::from("get user article success"),
                data: UserArticleType::Success(data),
            });
        }
        Err(err) => match err {
            sqlx::Error::RowNotFound => {
                return Ok(RtData {
                    success: true,
                    rt: 1,
                    msg: String::from("get user article success"),
                    data: UserArticleType::Success(Vec::new()),
                })
            }
            _ => {
                dbg!(err);
                return Err(Status::InternalServerError)
            },
        },
    }
}

#[post("/add_article", data = "<addArticleData>")]
pub fn add_article (mut db: BlogDBC, addArticleData: AddArticleData) {

}
