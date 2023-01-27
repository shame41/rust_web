use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive( Debug, Clone)]
pub struct Descriptor {
    pub title: String,
    pub view: i32,
    pub category: String,
    pub comment: i32,
    pub time: String,
    pub id: i32,
    pub tags: Vec<String>
}


