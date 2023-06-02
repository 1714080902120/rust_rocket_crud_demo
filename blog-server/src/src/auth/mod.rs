mod db_service;
mod fairing;
pub mod route;
pub mod token;
pub mod validate;

use crate::auth::token::set_token;
use rocket::{
    http::ContentType,
    response::{self, Responder, Response},
    FromForm, Request,
};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

use crate::{
    types::{LoginSuccessData, RtData},
};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct UserToken {
    pub id: String,
    pub exp: u64,
}

#[derive(Debug)]
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

impl RtData<LoginSuccessData> {
    fn hide_user_id (&mut self) {
        self.data.user_id = String::from("-");
    }
}

impl<'r> Responder<'r, 'static> for RtData<LoginSuccessData> {
    fn respond_to(mut self, req: &'r Request<'_>) -> response::Result<'static> {

        let user_id = self.data.user_id.as_str().to_owned();

        self.hide_user_id();

        let data = self.to_string();

        req.local_cache(|| AuthMsg {
            is_valid_token: true,
        });

        let (token_field, token) = set_token(req, user_id.as_str());

        Response::build()
            .header(ContentType::JSON)
            .raw_header(token_field.to_string(), token)
            .sized_body(data.len(), Cursor::new(data))
            .ok()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct UserExisted(());

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum RtDataType {
    Exist(UserExisted),
    Success(LoginSuccessData),
}

impl<'r> Responder<'r, 'static> for RtData<RtDataType> {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        let mut res = Response::build();
        res.header(ContentType::JSON);

        req.local_cache(|| AuthMsg {
            is_valid_token: true,
        });

        if let RtDataType::Success(login_data) = &self.data {
            let (token_field, token) = set_token(req, login_data.user_id.as_str());
            res.raw_header(token_field, token);
        };
        let data = self.to_string();
        res.sized_body(data.len(), Cursor::new(data)).ok()
    }
}
