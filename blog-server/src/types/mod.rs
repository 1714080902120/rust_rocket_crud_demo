use std::io::Cursor;

use jsonwebtoken::encode;
use rocket::{http::ContentType, response::Responder, Request, Response, response};
use serde::{Deserialize, Serialize};

use crate::{config::MyConfig, auth::{UserToken, token::encode_token}};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub author_id: i32,
    pub author_name: String,
    pub author_desc: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct FailureData(pub ());

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct ArticleData(pub Vec<Article>);

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct LoginSuccessData {
    pub name: String,
    pub desc: String,
    pub user_id: i32,
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
            .sized_body(data.len(), Cursor::new(data)).ok()
    }
}

impl<'r> Responder<'r, 'static> for RtData<ArticleData> {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
            
        let data = self.to_string();

        Response::build()
            .header(ContentType::JSON)
            .sized_body(data.len(), Cursor::new(data)).ok()
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
        

        Response::build()
            .header(ContentType::JSON)
            .raw_header(token_field, token)
            .sized_body(data.len(), Cursor::new(data)).ok()
    }
}