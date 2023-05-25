use crate::types::rt_type::Rt;
use rocket::http::Status;
use rocket::{form::Form, post, State};
use rocket_db_pools::sqlx::Row;

use crate::auth::{db_service::{RegisterRtType, get_user_msg}, LoginData};
use crate::db::{BlogDBC, SqlxError};
use crate::types::{LoginSuccessData, RtData};

use super::db_service::try_register_user;
use super::validate::{validate_login_data, validate_register_data, ValidateData};
use super::{RegisterData, RtDataType, UserExisted};

#[post("/login", data = "<login_data>")]
pub async fn login(
    db: BlogDBC,
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
        }
        Err(err) => {
            match err {
                SqlxError::RowNotFound => {
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
        rt: Rt::Success,
        data: user_msg,
    })
}

#[post("/register", data = "<register_data>")]
pub async fn register(
    db: BlogDBC,
    validator: &State<ValidateData>,
    register_data: Form<RegisterData>,
) -> Result<RtData<RtDataType>, Status> {
    validate_register_data(register_data.clone().into(), &validator)?;

    let res = try_register_user(db, register_data.into_inner()).await;
    
    match res {
        Ok(rt_type) => {
            dbg!(&rt_type);

            let mut success = true;
            let mut rt = Rt::Success;
            let mut msg = String::from("registry success !");

            let data = match rt_type {
                RegisterRtType::Exist(r_msg) => {
                    success = false;
                    rt = Rt::Fail;
                    msg = r_msg;
                    RtDataType::Exist(UserExisted(()))
                }
                RegisterRtType::Success(login_success_data) => {
                    RtDataType::Success(login_success_data)
                }
            };

            Ok(RtData {
                success,
                rt,
                msg,
                data,
            })
        }
        Err(err) => {
            dbg!(err);
            return Err(Status::InternalServerError);
        }
    }

}

// logout 由前端直接清除token即可。
