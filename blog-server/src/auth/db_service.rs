
use crate::{db::BlogDBC};
use rocket_db_pools::{
  sqlx::{self, postgres::PgRow},
};

pub async fn get_user_msg((login_key, pwd, is_email): (String, String, bool), mut db: BlogDBC) -> Result<PgRow, rocket_db_pools::sqlx::Error> {
    let condition = if is_email {
        format!("email = '{login_key}'")
    } else {
        format!("phone = {login_key}")
    };

    let pwd = format!("{:x}", md5::compute(pwd));

    let sql = format!("SELECT a.name, a.desc, a.id FROM public.user AS a WHERE a.pwd = '{pwd}' AND a.{condition}");
    dbg!(&sql);
    sqlx::query(&sql).fetch_one(&mut *db).await
}

pub async fn search_user_by_email_and_phone (email: &str, phone: &str, mut db: BlogDBC) -> Result<PgRow, rocket_db_pools::sqlx::Error> {
    let sql = format!("SELECT * FROM public.user WHERE email = '{email}' AND phone = {phone} LIMIT 1");

    sqlx::query(&sql).fetch_one(&mut *db).await

}
