mod db_service;
pub mod validate;
pub mod token;
mod fairing;
pub mod route;

use std::io::Cursor;
use jsonwebtoken::get_current_timestamp;
use rocket::{
    http::{ContentType},
    Request,
    response::{self, Responder, Response, Redirect}, FromForm,
    uri,
};
use serde::{Deserialize, Serialize};

use crate::{config::MyConfig, types::{RtData, LoginSuccessData}};

use self::{token::encode_token};
use crate::auth::route::login;

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
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, FromForm)]
pub struct RegisterData {
    #[field(name = "name")]
    name: String,
    #[field(name = "pwd")]
    pwd: String,
    #[field(name = "email")]
    email: String,
    #[field(name = "phone")]
    phone: String,
    #[field(name = "desc")]
    desc: String,
}

impl Into<(String, String, String, String, String)> for RegisterData {
    fn into(self) -> (String, String, String, String, String) {
        (self.name, self.pwd, self.email, self.phone, self.desc)
    }
}




impl<'r> Responder<'r, 'static> for RtData<LoginSuccessData> {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        let my_config = req.rocket().state::<MyConfig>().expect("get global state error when response in login");
        let token_field = my_config.token_field.as_str();
        let token_key = my_config.token_key.as_str();
        let expire_time = my_config.expire_time + get_current_timestamp();

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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct UserExisted(());


#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum RtDataType {
    Exist(UserExisted),
    Success(LoginSuccessData)
}


impl<'r> Responder<'r, 'static> for RtData<RtDataType> {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        match self.data {
            RtDataType::Exist(_) => {
                let data = self.to_string();
                Response::build().header(ContentType::JSON)
                .sized_body(data.len(), Cursor::new(data)).ok()
            },
            RtDataType::Success(_) => {
                Redirect::to(uri!(crate::auth::route::login()));

                let data = self.to_string();
                Response::build().header(ContentType::JSON)
                .sized_body(data.len(), Cursor::new(data)).ok()
            }
        }
    }
}