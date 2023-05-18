use regex::Regex;
use rocket::form::Form;
use rocket::{http::Status, State};
use serde::{Deserialize, Serialize};

use crate::config::MyConfig;

use super::LoginData;

#[derive(Debug, Clone)]
pub struct ValidateData {
    email_reg: Regex,
    phone_reg: Regex,
}

const EMAIL_REG_RULE: &'static str = r"^[a-zA-Z0-9_-]+@[a-zA-Z0-9_-]+(\.[a-zA-Z0-9_-]+)+$";
const PHONE_REG_RULE: &'static str = r"^1(3\d|4[5-9]|5[0-35-9]|6[567]|7[0-8]|8\d|9[0-35-9])\d{8}$";

impl ValidateData {
    pub fn new() -> Result<Self, Status> {
        let email_reg = match Regex::new(EMAIL_REG_RULE) {
            Ok(reg) => reg,
            Err(_) => return Err(Status::InternalServerError),
        };

        let phone_reg = match Regex::new(PHONE_REG_RULE) {
            Ok(reg) => reg,
            Err(_) => return Err(Status::InternalServerError),
        };

        Ok(Self {
            email_reg,
            phone_reg,
        })
    }

    pub fn is_empty(&self, target: Vec<&str>) -> bool {
        for item in target.iter() {
            if item.is_empty() {
                return true;
            }
        }
        return false;
    }

    pub fn validate_email(&self, target: &str) -> bool {
        match self.email_reg.captures(target) {
            Some(_) => true,
            None => {
                dbg!("validate email fail");
                false
            }
        }
    }

    pub fn validate_phone(&self, target: &str) -> bool {
        match self.phone_reg.captures(target) {
            Some(_) => true,
            None => {
                dbg!("validate phone fail");
                false
            }
        }
    }

    pub fn validate_name(&self, target: &str) -> bool {
        let len = target.len();
        if len <= 0 || len > 255 {
            return false;
        }
        return true;
    }

    pub fn validate_email_phone_name(&self, email: &str, phone: &str, name: &str) -> bool {
        return self.validate_email(email)
            && self.validate_phone(phone)
            && self.validate_name(name);
    }
}

pub fn validate_login_data(
    form_data: &mut Form<LoginData>,
    validator: &State<ValidateData>,
) -> Result<bool, Status> {
    let user_login_key = form_data.user_login_key.to_owned();
    let pwd = form_data.pwd.to_owned();

    dbg!(&form_data);

    if user_login_key.is_empty() || pwd.is_empty() {
        return Err(Status::BadRequest);
    }

    // vailidate

    let mut is_email = false;
    let mut is_fail = false;

    if user_login_key.contains("@") {
        is_email = true;
        is_fail = !validator.validate_email(&user_login_key);
    } else {
        is_fail = !validator.validate_phone(&user_login_key);
    };

    if is_fail {
        return Err(Status::BadRequest);
    }

    Ok(is_email)
}

pub fn validate_register_data(
    (name, pwd, email, phone, desc): (String, String, String, String, String),
    validator: &State<ValidateData>,
) -> Result<(), Status> {
    if validator.is_empty(vec![&name, &pwd, &email, &phone])
        || !validator.validate_email_phone_name(&email, &phone, &name)
    {
        return Err(Status::BadRequest);
    }

    Ok(())
}
