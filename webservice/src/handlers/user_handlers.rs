use crate::{state::{app_state::*, request_state, response_state}, errors::MyError};
use actix_web::{web, App, HttpResponse};
use serde_json::json;
use crate::models::descriptor::*;
use sqlx::Executor;
use crate::db::users::*;
use actix_web::{error, Result, Error};

//用户登录接口
// /api/user/login
pub async fn login_handler(
    app_state: web::Data<AppState>,//把服务器当作参数传入
    login_info: web::Json<request_state::LoginInfo>
    ) -> Result<HttpResponse, MyError> {
    println!("new request");
    post_user_login(
                    &app_state.db, 
                    login_info.email.clone(), 
                    login_info.password.clone()).await
        .map(|user| HttpResponse::Ok().json(json!({
                "code":200,
                "data":user
        })))
         


/*        HttpResponse::Ok().json(json!({
            "code": 200,
            "data": response_state::RegisterResponse {
                id: user_row.0.clone(),
                email: user_row.1.clone(),
                account_type: match user_row.3 {
                    false => "NORMAL".to_string(),
                    true  => "ADMIN".to_string()
                },
                avatar: user_row.2.clone() 
            }
        }))*/
    /*HttpResponse::Ok().json(json!({
        "code":200,
        "data":response_state::LoginResponse{
            nick_name: "nick_name".to_string(),
            user_id: 1,
            email: "email".to_string(),
            avatar: "avatar".to_string(),
            token: "token".to_string(),
            account_type: "account_type".to_string()
        }
    }))*/
    
}

//用户注册接口
// /api/user/register
/*pub async fn register_handler(
    app_state: web::Data<AppState>,//把服务器当作参数传入
    info: web::Json<request_state::RegisterInfo>,
    ) -> Result<HttpResponse, MyError>{

    println!("received new user");

    let user = post_user_register(
        &app_state.db, info.name.clone(), info.email.clone(), info.password.clone()).await;

    /*let new_user = User {
        id: new_user.user.id,
        name: "nickname",
        email: new_user.user.email,
        password: new_user.user.password,
        remember: true
    };*/

    Ok(HttpResponse::Ok().json(json!({
        "code": 200,
        "msg": "注册成功"
    })))
}*/

//获取用户信息
// get /api/user/register
pub async fn get_account_handler(
    app_state: web::Data<AppState>,//把服务器当作参数传入
    info: web::Query<request_state::AccountInfo>
    ) -> Result<HttpResponse, MyError> {
    println!("new request!");
    get_user_account(&app_state.db,
                                     info.email.clone(),
                                     info.password.clone())
        .await
        .map(|user| HttpResponse::Ok().json(json!({
            "code":200,
            "data": user
    })))
    
    /*Ok(HttpResponse::Ok().json(json!({
        "code":200,
        "data": user_info
    }))) */
}

//用户退出登录
//post /api/user/logout
pub async fn logout_handler(
    app_state: web::Data<AppState>,//把服务器当作参数传入
    ) -> Result<HttpResponse, MyError> {
/*    let articles: Iterator<Descriptor> = select_articals_by_page(
                                            &app_state.db,
                                            artical_info.current_page,
                                            artical_info.page_size);*/
    println!("new request!");

    Ok(HttpResponse::Ok().json(json!({
        "code":200,
        "msg": "退出登录成功"
    })))
}

