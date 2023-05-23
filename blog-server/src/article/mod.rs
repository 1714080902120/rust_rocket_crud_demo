mod db_service;
pub mod route;

use std::io::Cursor;

use rocket::{
    data::{self, Outcome, FromData, Data},
    http::{ContentType, Status},
    response::{self, Responder},
    Request, Response,
};
use serde::{Deserialize, Serialize};

use crate::{
    auth::{token::decode_token, AuthMsg},
    config::MyConfig,
    types::{ArticleData, RtData},
};

impl<'r> Responder<'r, 'static> for RtData<ArticleData> {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let data = self.to_string();

        Response::build()
            .header(ContentType::JSON)
            .sized_body(data.len(), Cursor::new(data))
            .ok()
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserAticleParams {
    user_id: String,
    article_id: String,
    all: bool,
}

#[rocket::async_trait]
impl<'r> FromData<'r> for UserAticleParams {
    type Error = String;
    async fn from_data(req: &'r Request<'_>, _: Data<'r>) -> data::Outcome<'r, Self> {
        use rocket::outcome::Outcome;
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
                let user_id = token.claims.id;
                let article_id = req.query_value("id").unwrap().unwrap();
                let all = req.query_value("all").unwrap().unwrap();
                return Outcome::Success(UserAticleParams {
                    user_id,
                    article_id,
                    all,
                });
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum UserArticleType {
    Success(Vec<UserArticle>),
    Fail
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserArticle {
    id: String,
    title: String,
    content: String,
}

impl<'r> Responder<'r, 'static> for RtData<UserArticleType> {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let data = self.to_string();

        Response::build()
            .header(ContentType::JSON)
            .sized_body(data.len(), Cursor::new(data))
            .ok()
    }
}


#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, FromForm)]
pub struct AddArticleData {
    #[field(name = "title")]
    title: String,
    #[field(name = "content")]
    content: String,
}

#[rocket::async_trait]
impl<'r> FromData<'r> for AddArticleData {
    type Error = String;
    async fn from_data(req: &'r Request<'_>, _: Data<'r>) -> data::Outcome<'r, Self> {
        use rocket::outcome::Outcome;
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
                let user_id = token.claims.id;
                let article_id = req.query_value("id").unwrap().unwrap();
                let all = req.query_value("all").unwrap().unwrap();
                return Outcome::Success(UserAticleParams {
                    user_id,
                    article_id,
                    all,
                });
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



