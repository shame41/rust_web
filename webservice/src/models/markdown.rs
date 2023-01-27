use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive( Debug, Clone)]
pub struct Markdown {
    pub id: i32,
    pub title: String,
    pub view: i32,
    pub favorite: i32,
    pub category: String,
    pub isPublish: bool,
    pub comment: i32,
    pub time: String,
    pub tags: String,
    pub text: String,
}
