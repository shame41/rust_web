use actix_web::{error, http::StatusCode, HttpResponse, Result};
use serde::Serialize;
use sqlx::error::Error as SQLxError;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum MyError {
    DBError(String),
    ActixError(String),
    NotFound(String),
}

#[derive(Debug, Serialize)]
pub struct MyErrorResponse {
    error_message: String,
}

impl MyError {
    fn error_response(&self) -> String {
        match self {
            MyError::DBError(msg) => {
                println!("数据库出现问题 : {:?}", msg);
                "Database error".to_string()
            }
            MyError::ActixError(msg) => {
                println!("服务器出现问题 : {:?}", msg);
                "Internal server error".to_string()
            }
            MyError::NotFound(msg) => {
                println!("Not found: {:?}",msg);
                msg.to_string()
            }
        }
    }
}

impl error::ResponseError for MyError {
    fn status_code(&self) -> StatusCode {
        match self {
            MyError::DBError(msg) | MyError::ActixError(msg) => StatusCode::INSUFFICIENT_STORAGE,
            MyError::NotFound(msg) => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
                        .json(MyErrorResponse{
                            error_message: self.error_response(),
                        })
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self) 
    }
}

//用于自动将actix_web的错误转换为MyError
impl From<actix_web::error::Error> for MyError {
    fn from(err: actix_web::error::Error) -> Self {
        MyError::ActixError(err.to_string()) 
    }
}

//用于自动将SQLx的错误转换为MyError
impl From<SQLxError> for MyError {
    fn from(err: SQLxError) -> Self {
        MyError::DBError(err.to_string()) 
    }
}
