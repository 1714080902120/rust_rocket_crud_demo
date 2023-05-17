mod db_service;
mod validate;
pub use validate::validate;
pub mod token;
mod fairing;
pub mod route;

use std::io::Cursor;
use rocket::{
    http::{ContentType},
    Request,
    response::{self, Responder, Response}, FromForm,
};
use serde::{Deserialize, Serialize};

use crate::{config::MyConfig, types::{RtData, LoginSuccessData}};

use self::token::encode_token;

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub struct UserToken {
    pub id: i32,
    pub expire_time: u64,
}

pub struct AuthMsg {
    pub is_valid_token: bool,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, FromForm)]
pub struct LoginData {
    #[field(name = "user_key")]
    pub user_login_key: String,
    #[field(name = "user_pwd")]
    pub pwd: String,
}

impl Into<(String, String)> for LoginData {
    fn into(self) -> (String, String) {
        (self.user_login_key, self.pwd)
    }
}


impl<'r> Responder<'r, 'static> for RtData<LoginSuccessData> {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        let my_config = req.rocket().state::<MyConfig>().expect("get global state error when response in login");
        let token_field = my_config.token_field.as_str();
        let token_key = my_config.token_key.as_str();
        let expire_time = my_config.expire_time;

        let user_id = self.data.user_id;


        let data = self.to_string();

        let token = encode_token(user_id, expire_time, token_key);
        
        req.local_cache(|| AuthMsg {
            is_valid_token: true
        });
        
        Response::build().header(ContentType::JSON)
        .raw_header(token_field.to_string(), token)
        .sized_body(data.len(), Cursor::new(data)).ok()
    }
}
