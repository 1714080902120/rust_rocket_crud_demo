mod db_service;
pub mod token;
mod fairing;
pub mod route;

use regex::Regex;
use rocket::{
    data::{FromData, Outcome},
    http::Status,
    request::FromRequest,
    Config, Data, Request,
};
use serde::{Deserialize, Serialize};

use crate::config::MyConfig;

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub struct UserToken {
    pub id: i32,
    pub expire_time: u64,
}

pub struct AuthMsg {
    pub is_valid_token: bool,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct LoginData {
    pub phone_or_email: String,
    pub pwd: String,
    pub is_email: bool,
}

impl Into<(String, String, bool)> for LoginData {
    fn into(self) -> (String, String, bool) {
        (self.phone_or_email, self.pwd, self.is_email)
    }
}

#[rocket::async_trait]
impl<'r> FromData<'r> for LoginData {
    type Error = &'static str;
    async fn from_data(req: &'r Request<'_>, _: Data<'r>) -> Outcome<'r, Self, Self::Error> {
        let mut phone_or_email = "";
        let mut pwd = "";

        for field in req.query_fields() {
            let name = field.name;

            match name.key() {
                Some(key) => {
                    let key: &str = key.as_ref();
                    if key.eq("user_key") {
                        phone_or_email = field.value;
                    } else if key.eq("user_pwd") {
                        pwd = key;
                    }
                }
                _ => (),
            };
        }

        if phone_or_email == "" || pwd == "" {
            return Outcome::Failure((Status::BadRequest, "param error"));
        }


        // vailidate

        let my_config = req
            .rocket()
            .state::<MyConfig>()
            .expect("get config error in login_data request guards");

        let mut is_email = false;
        let mut is_fail = false;

        if phone_or_email.contains("@") {
            is_email = true;
            let email_reg_rule = my_config.email_reg_rule.as_str();

            let email_reg = Regex::new(email_reg_rule).expect("parse email_reg_rule fail");

            if let None = email_reg.captures(phone_or_email) {
                is_fail = true;
            }

        } else {
            let phone_reg_rule = my_config.phone_reg_rule.as_str();
            let phone_reg = Regex::new(phone_reg_rule).expect("parse phone_reg_rule fail");

            if let None = phone_reg.captures(phone_or_email) {
                is_fail = true;
            }
        };

        if is_fail {
            return Outcome::Failure((Status::BadRequest, "vaildate user login key"));
        }



        Outcome::Success(Self {
            phone_or_email: phone_or_email.to_string(),
            pwd: pwd.to_string(),
            is_email,
        })
    }
}
