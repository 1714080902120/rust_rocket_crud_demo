use jsonwebtoken::get_current_timestamp;
use rocket_db_pools::{
    sqlx::{self, postgres::PgRow},
    Connection,
};

use crate::db::{Blog, BlogDBC};

fn get_article_sql(all: bool, art_id: &str) -> String {
    let mut sql = String::from("SELECT a.id, a.title, a.description, a.author_id, a.modify_time, b.name, b.description FROM public.article AS a LEFT JOIN public.user AS b ON a.author_id = b.id");
    if !all {
        sql += match art_id.is_empty() {
            true => {
                format!(" WHERE id = {art_id}")
            }
            _ => String::from(""),
        }
        .as_str()
    }
    sql
}

pub async fn get_article(
    mut db: Connection<Blog>,
    all: bool,
    id: &str,
) -> Result<Vec<PgRow>, sqlx::Error> {
    let sql: String = get_article_sql(all, id);
    sqlx::query(&sql).fetch_all(&mut *db).await
}

pub async fn try_get_user_article(
    mut db: Connection<Blog>,
    all: bool,
    user_id: &str,
    id: &str,
) -> Result<Vec<PgRow>, sqlx::Error> {
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
) -> Result<u64, (sqlx::Error, u64)> {
    let modify_time = get_current_timestamp();
    let sql = if is_add {
        format!("INSERT INTO public.article (id, title, author_id, description, modify_time) VALUES ('{article_id}', '{title}', '{author_id}', '{description}', {modify_time})")
    } else {
        format!("UPDATE public.article SET title = '{title}', description = '{description}', modify_time = {modify_time} WHERE id = '{article_id}'")
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
) -> Result<bool, sqlx::Error> {
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
                    sqlx::Error::RowNotFound => Ok(true),
                    _ => return Err(e),
                },
            }
        }
        Err(err) => match err {
            sqlx::Error::RowNotFound => Ok(false),
            _ => return Err(err),
        },
    }
}
