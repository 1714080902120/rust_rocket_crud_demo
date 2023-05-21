use rocket_db_pools::{
    sqlx::{self, postgres::PgRow},
    Connection
};

use crate::db::Blog;

fn get_article_sql(all: bool, art_id: &str) -> String {
    let mut sql = String::from("SELECT a.id, a.title, a.content, a.author_id, b.name, b.description FROM public.article AS a LEFT JOIN public.user AS b ON a.author_id = b.id");
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

pub async fn get_article(mut db: Connection<Blog>, all: bool, id: &str) -> Result<Vec<PgRow>, sqlx::Error> {
    let sql: String = get_article_sql(all, id);
    sqlx::query(&sql).fetch_all(&mut *db).await
}


