use rocket::form::Form;
use rocket::http::hyper::body::Buf;
use rocket_db_pools::sqlx::Row;
use std::fmt::format;
use std::path::Path;

use rocket::{get, http::Status, post, response::Redirect, tokio::fs, uri};
use uuid::Uuid;

use crate::article::db_service::{get_article, try_get_user_article};
use crate::article::UserArticleType;
use crate::db::BlogDBC;
use crate::types::{Article, ArticleData, RtData, UserMsg};

use super::db_service::save_article;
use super::{SetArticleData, UserArticle, UserAticleParams};

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
                description: row.get(2),
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
                    description: item.get(2),
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
                return Err(Status::InternalServerError);
            }
        },
    }
}

#[post("/set_article", data = "<setArticleData>")]
pub async fn set_article(
    mut db: BlogDBC,
    setArticleData: Form<SetArticleData>,
    user_msg: UserMsg,
) -> Result<(), Status> {
    let article = setArticleData.into_inner();

    let title = article.title;
    let content: &[u8] = article.content.as_ref();
    let user_id = user_msg.id;
    let article_id = if article.id.is_empty() {
        Uuid::new_v4().to_string()
    } else {
        article.id
    };

    let dir_path_name = format!("md/{user_id}");
    let dir_path = Path::new(&dir_path_name);
    match fs::read_dir(&dir_path).await {
        Ok(_) => (),
        Err(_) => {
            match fs::create_dir(dir_path).await {
                Ok(_) => (),
                Err(_) => {
                    return Err(Status::InternalServerError);
                }
            };
        }
    };

    let file_path_name = format!("md/{user_id}/{article_id}.{title}.md");

    match fs::write(Path::new(&file_path_name), content).await {
        Ok(_) => {
            let mut dst = [0; 200];
            content.copy_to_slice(&mut dst);
            save_article(
                db,
                article.id.is_empty(),
                user_id,
                article_id,
                title,
                String::from_utf8(dst.to_vec()),
            )
        }
        Err(err) => {
            return Err(Status::InternalServerError);
        }
    }

    Ok(())
}
