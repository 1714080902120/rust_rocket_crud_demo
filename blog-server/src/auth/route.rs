use rocket::http::Status;
use rocket::{
    post,
    State
};

use crate::auth::{LoginData, db_service::get_user_msg};
use crate::config::MyConfig;
use crate::db::BlogDBC;
use crate::types::{RtData, LoginSuccess};

#[post("/login", data = "<login_data>")]
pub async fn login(mut db: BlogDBC, login_data: LoginData, config: &State<MyConfig>) -> Result<RtData<LoginSuccess>, Status> {
    
    let result = get_user_msg(login_data.into(), db);

    match result {
        Ok(row) => {
            Ok(LoginSuccess {
                
            })
        }
        Err(_) => {
            Err(Status::InternalServerError)
        }
    }


}
