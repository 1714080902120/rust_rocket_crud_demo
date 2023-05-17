use rocket::{http::Status, State};
use regex::Regex;
use rocket::{ form::Form };

use crate::config::MyConfig;

use super::LoginData;

pub fn validate(form_data: &mut Form<LoginData>, my_config: &State<MyConfig>) -> Result<bool, Status> {
    let user_login_key = form_data.user_login_key.to_owned();
    let pwd = form_data.pwd.to_owned();

    dbg!(&form_data);

    if user_login_key == "" || pwd == "" {
        return Err(Status::BadRequest);
    }

    // vailidate

    let mut is_email = false;
    let mut is_fail = false;

    if user_login_key.contains("@") {
        is_email = true;
        let email_reg_rule = my_config.email_reg_rule.as_str();

        let email_reg = Regex::new(email_reg_rule).expect("parse email_reg_rule fail");
        if let None = email_reg.captures(&user_login_key) {
            is_fail = true;
        }
    } else {
        let phone_reg_rule = my_config.phone_reg_rule.as_str();
        let phone_reg = Regex::new(phone_reg_rule).expect("parse phone_reg_rule fail");
        if let None = phone_reg.captures(&user_login_key) {
            is_fail = true;
        }
    };

    if is_fail {
        return Err(Status::BadRequest);
    }

    Ok(is_email)
}
