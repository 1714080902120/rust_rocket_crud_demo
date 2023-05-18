
use crate::{db::BlogDBC};
use jsonwebtoken::get_current_timestamp;
use rocket_db_pools::{
  sqlx::{self, postgres::PgRow},
};
use uuid::Uuid;

use super::RegisterData;

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

pub async fn try_register_user (mut db: BlogDBC, register_data: RegisterData) -> Result<&'static str, rocket_db_pools::sqlx::Error> {
    let (name, pwd, email, phone, desc) = register_data.into();

    let email_c = email.as_str();
    let phone_c = phone.as_str();

    let sql = format!("SELECT * FROM public.user WHERE email = '{email_c}' AND phone = {phone_c} LIMIT 1");

    let res = if let Err(db_err) = sqlx::query(&sql).fetch_one(&mut *db).await {
        
        match db_err {
            rocket_db_pools::sqlx::Error::RowNotFound => {
                register_user(db, (name, pwd, email, phone, desc)).await?
            }
            _ => return Err(db_err)
        }
    } else {
        return Ok("phone or email had been registed !");
    };

    Ok(res)

}

pub async fn register_user(mut db: BlogDBC, (name, pwd, email, phone, desc): (String, String, String, String, String)) -> Result<&'static str, rocket_db_pools::sqlx::Error> {

    let (insert_key_suffix, insert_values_suffix) = if !desc.is_empty() {
        (", desc", format!(", '{desc}'"))
    } else {
        ("", String::new())
    };

    let id = Uuid::new_v4();    
    let pwd = format!("{:x}", md5::compute(pwd));
    let create_time = get_current_timestamp();

    let insert_key = format!("id, name, pwd, email, phone, create_time{insert_key_suffix}");
    let insert_values = format!("'{id}', '{name}', '{pwd}', '{email}', {phone}, {create_time}{insert_values_suffix}");

    let sql = format!("INSERT INTO public.user ({insert_key}) VALUES ({insert_values})");

    dbg!(&sql);

    match sqlx::query(&sql).fetch_one(&mut * db).await {
        Ok(_) => {
            Ok("registry successfully")
        }
        Err(err) => Err(err)
    }

}
