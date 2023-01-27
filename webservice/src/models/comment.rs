use super::user::*;

#[derive( Debug, Clone)]
pub struct Comment {
    pub user: User,
    pub content: String,
    pub time: String,
    pub articalid: i32
}
