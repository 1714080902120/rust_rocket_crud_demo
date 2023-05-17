mod db_service;
pub mod token;
mod fairing;
pub mod route;

use std::io::Cursor;
use regex::Regex;
use rocket::{
    data::{FromData, Outcome},
    http::{Status, ContentType},
    Data, Request,
    response::{self, Responder, Response}, FromForm, form::Errors
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
    #[field(name = "user_key", validate = validate_user_key(&(self.user_login_key)))]
    pub user_login_key: String,
    #[field(name = "user_pwd")]
    pub pwd: String,
    pub is_email: bool,
}

fn validate_user_key(user_login_key: &str) -> Result<(), Status> {
    if user_login_key.is_empty() {
        let err = Errors::new();
        err.append(Error);
    }
    Ok(())
}

impl Into<(String, String, bool)> for LoginData {
    fn into(self) -> (String, String, bool) {
        (self.user_login_key, self.pwd, self.is_email)
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

#[rocket::async_trait]
impl<'r> FromData<'r> for LoginData {
    type Error = &'static str;
    async fn from_data(req: &'r Request<'_>, mut form_data: Data<'r>) -> Outcome<'r, Self, Self::Error> {
        let mut user_login_key = "";
        let mut pwd = "";


        
        
        for field in req.query_fields() {
            let name = field.name;
            match name.key() {
                Some(key) => {
                    let key: &str = key.as_ref();
                    if key.eq("user_key") {
                        user_login_key = field.value;
                    } else if key.eq("user_pwd") {
                        pwd = key;
                    }
                }
                _ => (),
            };
        }

        if user_login_key == "" || pwd == "" {
            return Outcome::Failure((Status::BadRequest, "param error"));
        }


        // vailidate

        let my_config = req
            .rocket()
            .state::<MyConfig>()
            .expect("get config error in login_data request guards");

        let mut is_email = false;
        let mut is_fail = false;

        if user_login_key.contains("@") {
            is_email = true;
            let email_reg_rule = my_config.email_reg_rule.as_str();

            let email_reg = Regex::new(email_reg_rule).expect("parse email_reg_rule fail");

            if let None = email_reg.captures(user_login_key) {
                is_fail = true;
            }

        } else {
            let phone_reg_rule = my_config.phone_reg_rule.as_str();
            let phone_reg = Regex::new(phone_reg_rule).expect("parse phone_reg_rule fail");

            if let None = phone_reg.captures(user_login_key) {
                is_fail = true;
            }
        };

        if is_fail {
            return Outcome::Failure((Status::BadRequest, "vaildate user login key"));
        }



        Outcome::Success(Self {
            user_login_key: user_login_key.to_string(),
            pwd: pwd.to_string(),
            is_email,
        })
    }
}
