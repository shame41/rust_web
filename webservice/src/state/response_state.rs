use actix_web::web;
use serde::{Serialize, Deserialize};
use crate::models::user::*;

// #[derive(Serialize, Deserialize)]
/*pub struct LoginResponse {
    pub email: String,
    pub password: String,
    pub remember: String,
}*/

/*#[derive(Serialize, Deserialize)]
pub struct RegisterResponse {
    pub email: String,
    pub password: String
}*/

#[derive(Serialize, Deserialize)]
pub struct RegisterResponse {
    pub id: i32,
    pub email: String,
    pub account_type: String,
    pub avatar: String
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub nick_name: String,
    pub user_id: i32,
    pub email: String,
    pub avatar: String,
    pub token: String,
    pub account_type: String
}
/*impl From<web::Json<LoginResponse>> for LoginResponse {
    fn from(login_response: web::Json<LoginResponse>) -> Self {
        LoginResponse {
            nick_name: login_response.nick_name,
            user_id: login_response.user_id,
            email: login_response.email,
            avatar: login_response.avatar,
            token: login_response.token,
            account_type: login_response.account_type
        }
    }
}*/

#[derive(Serialize, Deserialize)]
pub struct UserInfoResponse {
    pub nick_name: String,
    pub user_id: i32,
    pub email: String,
    pub avatar: String,
    pub article_num: i32,
    pub read_num: i32,
    pub like_num: i32
}

#[derive(Serialize, Deserialize)]
pub struct AccountResponse {
    pub nick_name: String,
    pub user_id: i32,
    pub email: String,
    pub avatar: String,
    pub article_num: i32,
    pub read_num: i32,
    pub like_num: i32
}

pub struct Entity {
    pub view: i32,
    pub category: String,
    pub favorite: i32,
    pub comment: i32,
    pub tag: String,
    pub create_at: String,
    pub article_id: i32
}

#[derive(Serialize, Deserialize)]
pub struct Article {
    pub view: i32,
    pub category: String,
    pub favorite: i32,
    pub comment: i32,
    pub tags: Vec<String>,
    pub create_at: String,
    pub article_id: i32
}
#[derive(Serialize, Deserialize)]
pub struct ArticalsResponse {
    pub articles: Vec<Article>,
    pub article_count: i32
}

/*impl From<web::Json<ArticalsResponse>> for ArticalsResponse {
    fn from(article: web::Json<ArticalsResponse>) -> Self {
        ArticalsResponse {
            articles: article.articles,
            article_count: article.article_count,
        }
    }
}*/
#[derive(Serialize, Deserialize)]
pub struct DetailUser {
    pub avatar: String,
    pub nickname: String,
    pub total_view: i32,
    pub total_like: i32,
    pub total_comment: i32
}
#[derive(Serialize, Deserialize)]
pub struct DetailResponse {
    pub view: i32,
    pub content: String,
    pub author_id: i32,
    pub create_at: String,
    pub comment: i32,
    pub is_favorite: bool,
    pub favorite: i32,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub nickname: String,
    pub avatar: String 
}
#[derive(Serialize, Deserialize)]
pub struct Comment {
    pub author: User,
    pub content: String,
    pub create_at: String
}
#[derive(Serialize, Deserialize)]
pub struct CommentResponse {
    pub code: i32,
    pub data: Vec<Comment>,
}

#[derive(Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
    pub tag_id: i32
}

#[derive(Serialize, Deserialize)]
pub struct TagsResponse {
    pub data: Vec<Tag>,
}


#[derive(Serialize, Deserialize)]
pub struct Class {
    pub category_id: i32,
    pub name: String,
    pub amount: i32
}

#[derive(Serialize, Deserialize)]
pub struct ClassesResponse {
    pub data: Vec<Class>,
}

#[derive(Serialize, Deserialize)]
pub struct Draft {
    pub article_id: i32,
    pub content: String,
    pub is_publish: bool,
    pub updatad_at: String
}
#[derive(Serialize, Deserialize)]
pub struct DraftsResponse {
    pub code: i32,
    pub data: Vec<Draft>
}

#[derive(Serialize, Deserialize)]
pub struct DraftResponse {
    pub content: String,
    pub article_id: i32
}

#[derive(Serialize, Deserialize)]
pub struct DeleteDraftResponse {
    pub id: i32
}

#[derive(Serialize, Deserialize)]
pub struct CategoriesResponse {
    pub id: i32,
    pub name: String,
    pub amount: i32
}

#[derive(Serialize, Deserialize)]
pub struct CategoriesNum {
    pub name: String,
    pub number: i32
}

#[derive(Serialize, Deserialize)]
pub struct AdminArticles{
    pub id: i32,
    pub category: String,
    pub favorite: i32,
    pub view: i32,
    pub comment: i32,
    pub create_at: String

}
#[derive(Serialize, Deserialize)]
pub struct AdminArticlesResponse {
    pub articles: Vec<AdminArticles>,
    pub article_count: i32
}

#[derive(Serialize, Deserialize)]
pub struct CommentArticle {
    pub id: i32,
    pub title: String
}

#[derive(Serialize, Deserialize)]
pub struct CommentUser {
    pub nickname: String
}

#[derive(Serialize, Deserialize)]
pub struct Comments {
    pub id: i32,
    pub articles: CommentArticle,
    pub user: CommentUser,
    pub content: String,
    pub status: bool,
    pub create_at: String
}

#[derive(Serialize, Deserialize)]
pub struct AdminTag {
    pub id: i32,
    pub name: String,
    pub create_at: String
}

pub struct Entitys {
    pub articles: Vec<Entity>,
}

/* ===admin=== */
#[derive(Serialize, Deserialize)]
pub struct Category {
    pub category_id: i32,
    pub name: String,
    pub amount: i32 
}

#[derive(Serialize, Deserialize)]
pub struct GetCategoriesResponse{
    pub data: Vec<Category>,
}
