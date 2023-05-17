use rocket::http::Status;
use rocket::{form::Form, post, State};
use rocket_db_pools::sqlx::Row;


use crate::auth::{db_service::get_user_msg, LoginData};
use crate::config::MyConfig;
use crate::db::BlogDBC;
use crate::types::{LoginSuccessData, RtData};

use super::validate;

#[post("/login", data = "<login_data>")]
pub async fn login(
    db: BlogDBC,
    my_config: &State<MyConfig>,
    mut login_data: Form<LoginData>,
) -> Result<RtData<LoginSuccessData>, Status> {
    let user_login_key = login_data.user_login_key.to_owned();
    let pwd = login_data.pwd.to_owned();

    let is_email = validate(&mut login_data, &my_config)?;

    let result = get_user_msg((user_login_key, pwd, is_email), db).await;

    let user_msg = match result {
        Ok(row) => LoginSuccessData {
            name: row.get(0),
            desc: row.get(1),
            user_id: row.get(2),
        },
        Err(err) => {
            match err {
                rocket_db_pools::sqlx::Error::RowNotFound => {
                    // TODO 这里应该调到注册路由
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
    register_data: Form<LoginData>,
) -> Result<LoginSuccessData, Status> {
    let user_login_key = register_data.user_login_key.to_owned();
    let pwd = register_data.pwd.to_owned();

    let is_email = validate(&mut register_data, &my_config)?;

    let result = register_user().await;

}
