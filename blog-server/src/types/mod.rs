use std::io::Cursor;

use rocket::{
    data::{self, FromData},
    http::{ContentType, Status},
    request::{FromRequest, Outcome},
    response,
    response::Responder,
    Data, Request, Response,
};
use serde::{Deserialize, Serialize};

use crate::{
    auth::{token::decode_token, AuthMsg},
    config::MyConfig,
};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Article {
    pub id: String,
    pub title: String,
    pub modify_time: u64,
    pub description: String,
    pub author_name: String,
    pub author_desc: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct FailureData(pub ());

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct DefaultSuccessData(pub ());

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct ArticleData {
    pub list: Vec<Article>,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct LoginSuccessData {
    pub name: String,
    pub desc: String,
    pub user_id: String,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct RtData<T> {
    pub success: bool,
    pub rt: i32,
    pub data: T,
    pub msg: String,
}

impl<T: Serialize> RtData<T> {
    pub fn to_string(mut self) -> String {
        serde_json::to_string(&mut self).unwrap()
    }
}

impl<'r> Responder<'r, 'static> for RtData<FailureData> {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let data = self.to_string();

        Response::build()
            .header(ContentType::JSON)
            .sized_body(data.len(), Cursor::new(data))
            .ok()
    }
}

impl<'r> Responder<'r, 'static> for RtData<DefaultSuccessData> {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let data = self.to_string();

        Response::build()
            .header(ContentType::JSON)
            .sized_body(data.len(), Cursor::new(data))
            .ok()
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserMsg {
    pub id: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserMsg {
    type Error = String;
    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if req
            .local_cache(|| AuthMsg {
                is_valid_token: false,
            })
            .is_valid_token
        {
            let my_config = req
                .rocket()
                .state::<MyConfig>()
                .expect("get global custom config error in fairing");
            let token_field = my_config.token_field.as_str();
            let token_key = my_config.token_key.as_str();

            let header = req.headers();
            let token_data = header.get(token_field).next();
            if let Some(token) = token_data {
                let token = decode_token(token, token_key).unwrap();
                let id = token.claims.id;
                return Outcome::Success(UserMsg { id });
            } else {
                return Outcome::Failure((
                    Status::BadRequest,
                    String::from("user no login or token expired"),
                ));
            }
        } else {
            return Outcome::Failure((
                Status::BadRequest,
                String::from("user no login or token expired"),
            ));
        }
    }
}
