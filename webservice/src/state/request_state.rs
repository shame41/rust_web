use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct LoginInfo {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterInfo {
    pub name: String,
    pub email: String,
    pub password: String
}

#[derive(Serialize, Deserialize)]
pub struct AccountInfo {
    pub email: String,
    pub password: String
}

#[derive(Serialize, Deserialize)]
pub struct ArticalInfo {
    pub current_page: i32,
    pub page_size: i32,
    pub keyword: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ArticalDetail {
    pub article_id: i32,
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct CommentInfo {
    pub article_id: i32,
    pub token: String
}

#[derive(Serialize, Deserialize)]
pub struct AddComment {
    pub articleId: i32,
    pub content: String,
    pub author: i32
}

#[derive(Serialize, Deserialize)]
pub struct FavoriteInfo {
    pub articleId: i32,
    pub userId: i32
}

#[derive(Serialize, Deserialize)]
pub struct IsFavoriteInfo {
    pub articleId: i32,
}

#[derive(Serialize, Deserialize)]
pub struct SaveDraftInfo {
    pub markdown: String,
    pub title: String
}

#[derive(Serialize, Deserialize)]
pub struct DraftInfo {
    pub article_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateDraft {
    pub markdown: String,
    pub title: String,
    pub id: i32
}

#[derive(Serialize, Deserialize)]
pub struct DeleteDraft {
    pub article_id: i32
}

#[derive(Serialize, Deserialize)]
pub struct Publish {
    pub content: String,
    pub tags: Vec<i32>,
    pub category: i32
}

#[derive(Serialize, Deserialize)]
pub struct ArticalList {
    pub page: i32,
    pub page_size: i32
}

#[derive(Serialize, Deserialize)]
pub struct ArticalByClass {
    pub currentPage: i32,
    pub pageSize: i32,
    pub class1: String
}

#[derive(Serialize, Deserialize)]
pub struct ArticalByTag {
    pub currentPage: i32,
    pub pageSize: i32,
    pub tag: String
}

#[derive(Serialize, Deserialize)]
pub struct createTagInfo {
    pub name: String
}

#[derive(Serialize, Deserialize)]
pub struct CategoryInfo {
    pub category_id: i32,
    pub name: String,
    pub amount: i32
}
