use crate::state::app_state::*;
use actix_web::{web, App, HttpResponse};
use crate::models::user;
use serde_json::json;

pub async fn first_handler() -> HttpResponse{

    println!("bingo!");
    HttpResponse::Ok().json(json!({
        "code":200,
        "msg":"hello"
    }))
}
