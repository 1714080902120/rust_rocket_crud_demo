use std::io::Cursor;

use rocket::{http::ContentType, response::Responder, Request, Response, response};
use serde::{Deserialize, Serialize};
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Article {
    pub id: String,
    pub title: String,
    pub content: String,
    pub author_id: String,
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
