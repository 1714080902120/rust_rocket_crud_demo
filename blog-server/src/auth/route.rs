use rocket::http::Status;
use rocket::{post};
use rocket::form::Form;
use rocket_db_pools::sqlx::Row;

use crate::auth::{db_service::get_user_msg, LoginData};
use crate::db::BlogDBC;
use crate::types::{LoginSuccessData, RtData};

#[post("/login", data = "<login_data>")]
pub async fn login(db: BlogDBC, login_data: LoginData) -> Result<RtData<LoginSuccessData>, Status> {
    let result = get_user_msg(login_data.into(), db).await;

    let user_msg = match result {
        Ok(row) => LoginSuccessData {
            name: row.get(0),
            desc: row.get(1),
            user_id: row.get(2),
        },
        Err(_) => return Err(Status::InternalServerError),
    };


    Ok(RtData {
        success: true,
        msg: String::from("login success"),
        rt: 111,
        data: user_msg,
    })
}
