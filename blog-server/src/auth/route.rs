use rocket::http::Status;
use rocket::{form::Form, post, response::Redirect, uri, State};
use rocket_db_pools::sqlx::Row;
use uuid::{Uuid, Bytes};

use crate::auth::{db_service::get_user_msg, LoginData};
use crate::config::MyConfig;
use crate::db::BlogDBC;
use crate::types::{LoginSuccessData, RtData};

use super::db_service::try_register_user;
use super::validate::{validate_login_data, validate_register_data, ValidateData};
use super::{RegisterData, RtDataType, UserExisted};

#[post("/login", data = "<login_data>")]
pub async fn login(
    db: BlogDBC,
    my_config: &State<MyConfig>,
    validator: &State<ValidateData>,
    mut login_data: Form<LoginData>,
) -> Result<RtData<LoginSuccessData>, Status> {
    let user_login_key = login_data.user_login_key.to_owned();
    let pwd = login_data.pwd.to_owned();

    let is_email = validate_login_data(&mut login_data, &validator)?;

    let result = get_user_msg((user_login_key, pwd, is_email), db).await;

    let user_msg = match result {
        Ok(row) => {
            let user_id = row.get::<sqlx::types::Uuid, usize>(2).to_string();
            LoginSuccessData {
                name: row.get(0),
                desc: row.get(1),
                user_id,
            }
        },
        Err(err) => {
            match err {
                rocket_db_pools::sqlx::Error::RowNotFound => {
                    dbg!("row not found");
                    return Err(Status::BadRequest);
                }
                _ => {
                    let db_err = err.into_database_error().expect("is not db err");
                    dbg!(db_err.message());
                    return Err(Status::InternalServerError);
                }
            };
        }
    };

    Ok(RtData {
        success: true,
        msg: String::from("login success"),
        rt: 111,
        data: user_msg,
    })
}

#[post("/register", data = "<register_data>")]
pub async fn register(
    db: BlogDBC,
    my_config: &State<MyConfig>,
    validator: &State<ValidateData>,
    register_data: Form<RegisterData>,
) -> Result<RtData<RtDataType>, Status> {
    validate_register_data(register_data.clone().into(), &validator)?;

    let res = try_register_user(db, register_data.into_inner()).await;

    match res {
        Ok(s) => {
            dbg!(s);
        }
        Err(err) => {
            dbg!(err);
        }
    };

    Redirect::to(uri!(crate::auth::route::login()));

    Ok(RtData {
        success: false,
        rt: -33,
        msg: String::from("phone or email had been registered !"),
        data: RtDataType::Exist(UserExisted(())),
    })
}
