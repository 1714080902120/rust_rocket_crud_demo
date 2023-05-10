
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub desc: String,
    pub reg_time: i32,
    pub email: String,
    pub phone: i32,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Article {
    pub id: usize,
    pub title: String,
    pub content: String,
    pub author_name: String,
    pub author_desc: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct RtData<T> {
    pub success: bool,
    pub msg: String,
    pub data: T,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct FailureData(pub ());

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct AllArticleData(Article);

pub type SucAllArticle = RtData<AllArticleData>;
pub type FailureAllArticle = RtData<FailureData>;
