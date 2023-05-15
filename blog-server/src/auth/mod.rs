mod fairing;
pub mod route;

use rocket::{
    data::{FromData, Outcome},
    http::{ Status},
    Data, Request
};
use serde::{Deserialize, Serialize};




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

        Outcome::Success(Self {
            phone_or_email: phone_or_email.to_string(),
            pwd: pwd.to_string(),
        })
    }
}


