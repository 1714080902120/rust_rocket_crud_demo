use std::io::Cursor;

use rocket::{http::ContentType, response::Responder, Request, Response, response};
use serde::{Deserialize, Serialize};

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

impl<'r, T: Serialize> Responder<'r, 'static> for RtData<T> {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let data = self.to_string();

        Response::build()
            .header(ContentType::JSON)
            .sized_body(data.len(), Cursor::new(data)).ok()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct LoginSuccess {
    name: String,
    desc: String,
    id: i32,
}
