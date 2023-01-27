use actix_web::web;
use serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub remember: bool
}


/*impl From<web::Json<User>> for User {//解析进入的json报文
      fn from(user: web::Json<User>) -> Self {
          User {
              email: user.email.clone(),
              password: user.password.clone(),
              remember: true
          }
      }
}*/
