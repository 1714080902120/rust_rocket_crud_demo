use jsonwebtoken::get_current_timestamp;
use rocket_db_pools::{
    sqlx::{self, postgres::PgRow},
};

use crate::db::{BlogDBC, SqlxError, DbQueryResult};

pub async fn get_article(
    mut db: BlogDBC,
    all: bool,
    id: &str,
    limit: i32,
    page_no: i32,
) -> DbQueryResult<Vec<PgRow>> {
    let mut sql = String::from("SELECT a.id, a.title, a.description, a.author_id, a.modify_time, b.name, b.description FROM public.article AS a LEFT JOIN public.user AS b ON a.author_id = b.id WHERE a.is_publish = true");
    let offset = page_no * limit;
    if !all {
        sql += match id.is_empty() {
            true => {
                format!(" AND id = {id}")
            }
            _ => String::from(""),
        }
        .as_str();
    }
    sql += format!(" LIMIT = {limit} OFFSET = {offset}").as_str();
    sqlx::query(&sql).fetch_all(&mut *db).await
}

pub async fn try_get_user_article(
    mut db: BlogDBC,
    all: bool,
    user_id: &str,
    id: &str,
) -> DbQueryResult<Vec<PgRow>> {
    let sql =
        format!("SELECT id, title, description, modify_time FROM public.article WHERE author_id = '{user_id}'")
            + if all {
                format!("")
            } else {
                format!(" AND id = {id}")
            }
            .as_str();

    let res = sqlx::query(&sql).fetch_all(&mut *db).await?;
    Ok(res)
}

pub async fn save_article(
    mut db: BlogDBC,
    is_add: bool,
    author_id: String,
    article_id: String,
    title: String,
    description: &str,
    is_publish: bool,
) -> Result<u64, (SqlxError, u64)> {
    let modify_time = get_current_timestamp();
    let sql = if is_add {
        format!("INSERT INTO public.article (id, title, author_id, description, modify_time, is_publish) VALUES ('{article_id}', '{title}', '{author_id}', '{description}', {modify_time}, {is_publish})")
    } else {
        format!("UPDATE public.article SET title = '{title}', description = '{description}', modify_time = {modify_time}, is_publish = {is_publish} WHERE id = '{article_id}'")
    };

    dbg!(&sql);

    match sqlx::query(&sql).fetch_one(&mut *db).await {
        Ok(_) => Ok(modify_time),
        Err(err) => Err((err, modify_time)),
    }
}

pub async fn try_delete_article(
    mut db: BlogDBC,
    id: String,
    author_id: String,
) -> DbQueryResult<bool> {
    let query_sql = format!(
        "SELECT * FROM public.article WHERE id = '{id}' AND author_id = '{author_id}' LIMIT 1"
    );

    match sqlx::query(&query_sql).fetch_one(&mut *db).await {
        Ok(_) => {
            let sql = format!(
                "DELETE FROM public.article WHERE id = '{id}' AND author_id = '{author_id}'"
            );
            dbg!(&sql);
            match sqlx::query(&sql).fetch_one(&mut *db).await {
                Ok(_) => Ok(true),
                Err(e) => match e {
                    SqlxError::RowNotFound => Ok(true),
                    _ => return Err(e),
                },
            }
        }
        Err(err) => match err {
            SqlxError::RowNotFound => Ok(false),
            _ => return Err(err),
        },
    }
}
