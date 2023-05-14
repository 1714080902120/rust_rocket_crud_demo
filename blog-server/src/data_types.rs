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
pub struct AllArticleData(pub Vec<Article>);

// pub struct UserId {
//     pub id: i32
// }
// #[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
// pub struct UserToken {
//     pub id: i32,
//     pub expire_time: i64
// }

// const KEY: &'static str = "dan";

// #[rocket::async_trait]
// impl<'r> FromRequest<'r> for UserId {
//     type Error = Status;
//     async fn from_request(request: &'r rocket::Request<'_>) ->  Outcome<Self, Self::Error> {
//         let header = request.headers();
//         if let Some(token) = header.get("token").next() {
//             let token_msg = decode::<UserToken>(token, &DecodingKey::from_secret(&KEY.as_ref()), &Validation::new(jsonwebtoken::Algorithm::ES256));
//             match token_msg {
//                 Ok(token_data) => {
//                     if let Some(id) = request.query_value("id") {
//                         match id {
//                             Ok(user_id) => {

//                             }
//                             Err(err) => {

//                             }
//                         }
//                     }
//                 }
//                 Err(err) => {

//                     Outcome::Failure(Status::Unauthorized)
//                 }
//             }
//         }
//         Outcome::Success(Self {
//             id: request.g
//         })

//     }
// }
