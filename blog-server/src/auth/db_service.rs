use crate::{db::BlogDBC};
use rocket_db_pools::{
  sqlx::{self, postgres::PgRow},
};

pub async fn get_user_msg((login_key, pwd, is_email): (String, String, bool), mut db: BlogDBC) -> Result<PgRow, rocket_db_pools::sqlx::Error> {
    let condition = if is_email {
        format!("email = {login_key}")
    } else {
        format!("phone = {login_key}")
    };

    let sql = format!("SELECT name, desc, id FROM public.user WHERE pwd = {pwd} AND {condition}");
    sqlx::query(sql.as_str()).fetch_one(&mut *db).await
}
