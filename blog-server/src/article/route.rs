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
use super::{SetArticleData, SetArticleDataState, UserArticle, UserAticleParams};

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
                modify_time: row.get::<i64, usize>(4) as u64,
                author_name: row.get(5),
                author_desc: row.get(6),
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
                    modify_time: item.get::<i64, usize>(3) as u64,
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

#[post("/set_article", data = "<set_article_data>")]
pub async fn set_article(
    db: BlogDBC,
    set_article_data: Form<SetArticleData>,
    user_msg: UserMsg,
) -> Result<RtData<SetArticleDataState>, Status> {
    let article = set_article_data.into_inner();

    let title = article.title;
    let save_content = format!("# {title}\n") + article.content.as_str();
    let content_buf: &[u8] = save_content.as_ref();
    let user_id = user_msg.id;
    let is_add = article.id.is_empty();
    let article_id = if is_add {
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

    let file_path_name = format!("md/{user_id}/{article_id}.md");

    match fs::write(Path::new(&file_path_name), content_buf).await {
        Ok(_) => {
            let content_len = article.content.len();
            let dst = if content_len < 200 {
                article.content.as_str()
            } else {
                article.content.get(0..=200).unwrap()
            };
            match save_article(
                db,
                is_add,
                user_id,
                article_id,
                title,
                dst,
            )
            .await
            {
                Ok(modifiy_time) => {
                    return Ok(RtData {
                        success: true,
                        rt: 44,
                        msg: String::from("set article success"),
                        data: SetArticleDataState::Success(modifiy_time),
                    });
                }
                Err((err, _modify_time)) => {
                    dbg!(&err);
                    match err {
                        sqlx::Error::RowNotFound => {
                            return Ok(RtData {
                                success: true,
                                rt: 44,
                                msg: String::from("set article success"),
                                data: SetArticleDataState::Success(_modify_time),
                            });
                        },
                        _ => return Ok(RtData {
                            success: false,
                            rt: -44,
                            msg: String::from("set article fail"),
                            data: SetArticleDataState::Fail(()),
                        })
                    };
                    
                }
            }
        }
        Err(err) => {
            dbg!(err);
            return Err(Status::InternalServerError);
        }
    };
}
