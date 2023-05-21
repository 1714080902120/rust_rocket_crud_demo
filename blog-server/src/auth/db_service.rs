use crate::{db::BlogDBC, types::LoginSuccessData};
use jsonwebtoken::get_current_timestamp;
use rocket_db_pools::sqlx::{self, postgres::PgRow};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::RegisterData;

#[derive(Debug, Serialize, Deserialize)]
pub enum RegisterRtType {
    Exist(String),
    Success(LoginSuccessData),
}

pub async fn get_user_msg(
    (login_key, pwd, is_email): (String, String, bool),
    mut db: BlogDBC,
) -> Result<PgRow, rocket_db_pools::sqlx::Error> {
    let condition = if is_email {
        format!("email = '{login_key}'")
    } else {
        format!("phone = {login_key}")
    };

    let pwd = format!("{:x}", md5::compute(pwd));

    let sql = format!(
        "SELECT name, description, id FROM public.user WHERE pwd = '{pwd}' AND {condition}"
    );
    dbg!(&sql);
    sqlx::query(&sql).fetch_one(&mut *db).await
}

pub async fn try_register_user(
    mut db: BlogDBC,
    register_data: RegisterData,
) -> Result<RegisterRtType, rocket_db_pools::sqlx::Error> {
    let (name, pwd, email, phone, desc) = register_data.into();

    let email_c = email.as_str();
    let phone_c = phone.as_str();

    let sql =
        format!("SELECT * FROM public.user WHERE email = '{email_c}' OR phone = {phone_c} LIMIT 1");

    let res = if let Err(db_err) = sqlx::query(&sql).fetch_one(&mut *db).await {
        match db_err {
            rocket_db_pools::sqlx::Error::RowNotFound => {
                register_user(db, (name, pwd, email, phone, desc)).await?
            }
            _ => return Err(db_err),
        }
    } else {
        return Ok(RegisterRtType::Exist(String::from(
            "email or phone had been registried !",
        )));
    };

    Ok(res)
}

pub async fn register_user(
    mut db: BlogDBC,
    (name, pwd, email, phone, desc): (String, String, String, String, String),
) -> Result<RegisterRtType, rocket_db_pools::sqlx::Error> {
    let id = Uuid::new_v4();
    let pwd = format!("{:x}", md5::compute(pwd));
    let create_time = get_current_timestamp();

    let desc = if desc.is_empty() {
        String::from("这人很懒，什么都没留下~")
    } else {
        desc
    };

    let insert_key = format!("id, name, pwd, email, phone, description, create_time");
    let insert_values =
        format!("'{id}', '{name}', '{pwd}', '{email}', {phone}, '{desc}', {create_time}");

    let sql = format!("INSERT INTO public.user ({insert_key}) VALUES ({insert_values})");

    dbg!(&sql);

    match sqlx::query(&sql).fetch_one(&mut *db).await {
        Ok(_) => Ok(RegisterRtType::Success(LoginSuccessData {
            name,
            desc,
            user_id: id.to_string(),
        })),
        Err(err) => {
            if let sqlx::Error::RowNotFound = err {
                return Ok(RegisterRtType::Success(LoginSuccessData {
                    name,
                    desc,
                    user_id: id.to_string(),
                }));
            }
            return Err(err);
        }
    }
}
