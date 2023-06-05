use rocket::form::Form;
use rocket::State;
use rocket_db_pools::sqlx::Row;
use std::path::Path;

use rocket::{delete, get, http::Status, post, response::Redirect, tokio::fs, uri};
use uuid::Uuid;

use crate::article::db_service::{get_article, try_get_user_article};
use crate::article::UserArticleType;
use crate::config::MyConfig;
use crate::db::{is_row_not_found, BlogDBC, SqlxError};
use crate::types::rt_type::Rt;
use crate::types::{Article, ArticleData, DefaultSuccessData, GetArticleData, RtData, UserMsg};

use super::db_service::{article_detail, save_article, try_delete_article, try_search_article};
use super::file_operate::{read_md_into_str, write_into_md};
use super::{
    ArticleDetail, ArticleDetailData, SetArticleData, SetArticleDataState, UserArticle,
    UserAticleParams,
};

#[get("/")]
pub fn index() {
    Redirect::to(uri!("/get_article?all=true&id=-1&author_id=-1"));
}

/// get article
#[get("/get_article?<all>&<id>&<limit>&<page_no>")]
pub async fn route_article(
    db: BlogDBC,
    all: bool,
    id: &str,
    limit: i32,
    page_no: i32,
) -> Result<RtData<ArticleData>, Status> {
    let limit = if limit < 0 { 20 } else { limit };

    let result = get_article(db, all, id, limit, page_no).await;

    match result {
        Ok(v) => {
            let articles = v.iter().map(|row| Article {
                id: row.get::<Uuid, usize>(0).to_string(),
                title: row.get(1),
                description: row.get(2),
                modify_time: row.get::<i64, usize>(4) as u64,
                author_name: row.get(5),
                author_desc: row.get(6),
            });

            Ok(RtData {
                success: true,
                msg: String::from("get all article success!"),
                rt: Rt::Success,
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
                rt: Rt::Success,
                msg: String::from("get user article success"),
                data: UserArticleType::Success(data),
            });
        }
        Err(err) => match err {
            SqlxError::RowNotFound => {
                return Ok(RtData {
                    success: true,
                    rt: Rt::Success,
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
    my_config: &State<MyConfig>,
) -> Result<RtData<SetArticleDataState>, Status> {

    let article = set_article_data.into_inner();

    let title = article.title;
    let description = article.description;
    let save_content = format!("# {title}\n") + article.content.as_str();
    let content_buf: &[u8] = save_content.as_ref();
    let user_id = user_msg.id;
    let is_publish = article.is_publish;
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

    write_into_md(&user_id, &article_id, content_buf).await?;


    match save_article(db, is_add, user_id, article_id, title, &description, is_publish).await {
        Ok(modifiy_time) => {
            return Ok(RtData {
                success: true,
                rt: Rt::Success,
                msg: String::from("set article success"),
                data: SetArticleDataState::Success(modifiy_time),
            });
        }
        Err((err, _modify_time)) => {
            dbg!(&err);
            match err {
                SqlxError::RowNotFound => {
                    return Ok(RtData {
                        success: true,
                        rt: Rt::Success,
                        msg: String::from("set article success"),
                        data: SetArticleDataState::Success(_modify_time),
                    });
                }
                _ => {
                    return Ok(RtData {
                        success: false,
                        rt: Rt::Fail,
                        msg: String::from("set article fail"),
                        data: SetArticleDataState::Fail(()),
                    })
                }
            };
        }
    }
}

#[delete("/del_article?<id>")]
pub async fn del_article(
    db: BlogDBC,
    id: String,
    user_msg: UserMsg,
) -> Result<RtData<DefaultSuccessData>, Status> {
    let user_id = user_msg.id;

    match try_delete_article(db, id, user_id).await {
        Ok(state) => {
            if state {
                Ok(RtData {
                    success: true,
                    rt: Rt::Success,
                    data: DefaultSuccessData(()),
                    msg: String::from("delete success !"),
                })
            } else {
                Ok(RtData {
                    success: false,
                    rt: Rt::Fail,
                    data: DefaultSuccessData(()),
                    msg: String::from("delete fail, 有内鬼，终止交易!"),
                })
            }
        }
        Err(err) => {
            dbg!(err);
            return Err(Status::InternalServerError);
        }
    }
}

#[get("/article_detail?<id>")]
pub async fn get_article_detail(
    db: BlogDBC,
    id: &str,
    user_msg: UserMsg,
) -> Result<RtData<ArticleDetailData>, Status> {
    match article_detail(db, id).await {
        Ok(row) => {
            let author_id: String = row.get::<Uuid, usize>(3).to_string();
            let content = read_md_into_str(&user_msg.id, &id).await?;

            let article_detail = ArticleDetail {
                id: row.get::<Uuid, usize>(0).to_string(),
                title: row.get(1),
                content,
                modify_time: row.get::<i64, usize>(2) as u64,
                can_edit: author_id == user_msg.id,
            };
            Ok(RtData {
                success: true,
                rt: Rt::Success,
                data: ArticleDetailData::Success(article_detail),
                msg: String::from("get detail success"),
            })
        }
        Err(err) => {
            if is_row_not_found(err) {
                return Ok(RtData {
                    success: false,
                    rt: Rt::Fail,
                    data: ArticleDetailData::Fail,
                    msg: String::from("not found"),
                });
            } else {
                return Err(Status::InternalServerError);
            }
        }
    }
}

#[get("/search?<condition>")]
pub async fn search_article(
    db: BlogDBC,
    condition: &str,
) -> Result<RtData<GetArticleData>, Status> {
    match try_search_article(db, condition).await {
        Ok(rows) => Ok(RtData {
            success: true,
            rt: Rt::Success,
            data: GetArticleData::Success(ArticleData {
                list: rows.iter().map(|row| Article {
                    id: row.get::<Uuid, usize>(0).to_string(),
                    title: row.get(1),
                    modify_time: row.get::<i64, usize>(2) as u64,
                    description: row.get(3),
                    author_name: row.get(4),
                    author_desc: row.get(5),
                }).collect(),
            }),
            msg: String::from("search success"),
        }),
        Err(err) => {
            dbg!(&err);
            if is_row_not_found(err) {
                Ok(RtData {
                    success: false,
                    rt: Rt::Fail,
                    msg: String::from("not found"),
                    data: GetArticleData::Fail,
                })
            } else {
                return Err(Status::InternalServerError);
            }
        }
    }
}
