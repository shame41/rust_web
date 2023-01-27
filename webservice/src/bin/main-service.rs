use actix_web::{web, error, get, middleware::Logger, App, HttpServer, Result};
use sea_orm::Database;
use std::io;
use std::env;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use actix_cors::Cors;
use log::info;


#[path = "../handlers/mod.rs"]
mod handlers;
#[path = "../routers.rs"]
mod routers;
#[path = "../state/mod.rs"]
mod state;
#[path = "../models/mod.rs"]
mod models;
#[path = "../db/mod.rs"]
mod db;
#[path = "../errors.rs"]
mod errors;

use crate::models::{user::*, comment::*, markdown::*, descriptor::*};
use state::app_state::AppState;
use routers::*;

#[actix_rt::main]
async fn main() -> io::Result<()>{
    dotenv().ok();
    println!("初始化数据库中");
    let database_url = env::var("DATABASE_URL").expect("database_url is wrong");
    let db_pool = PgPoolOptions::new()
                    .connect(&database_url).await.unwrap();
    // let db_pool: DatabaseConnection = Database::connect(&database_url).await?;
    println!("数据库连接成功");

    let app_state = web::Data::new(AppState {
        db: db_pool,
    }); 
    

    let app = move || {
        App::new()
                .wrap(Cors::permissive())
                .app_data(app_state.clone())
                .configure(routers::general_routes) 
                .configure(routers::user_routes)
                .configure(routers::article_routes)
                .configure(routers::write_routes)
                .configure(routers::admin_routes)
    };//初始化服务器，注册路由
    println!("启动服务器");
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await//运行服务器
}
