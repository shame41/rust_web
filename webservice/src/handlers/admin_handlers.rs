use actix_web::{web, App, HttpResponse};
use crate::errors::MyError;
use crate::models::{descriptor::*, markdown::*};
use crate::state::{app_state::*, request_state, response_state};
use serde_json::json;
use crate::db::admins::*;

//获取文章列表
//get /api/admin/articles
/*pub async fn articles_handler(
    app_state: web::Data<AppState>,//把服务器当作参数传入
    request: web::Query<request_state::ArticalList>
    ) -> Result<HttpResponse, MyError> {
    let article = get_article_articles(
                                        &app_state.db,
                                        request.current_page,
                                        request.page_size).await?;
    println!("new request!"); 
    let response: Vec<response_state::AdminArticles> = articles_row.iter()
                    .map(|article| 
                         response_state::AdminArticles{
                            id: article.0,
                            category: article.2.clone(),
                            favorite: article.3,
                            view: article.4,
                            comment: article.5,
                            create_at: article.6.to_string()
                }).collect();

    let len = response.len() as i32;
    Ok(HttpResponse::Ok().json(json!({
        "code": 200,
        "data": response_state::AdminArticlesResponse{
            articles: article,
            article_count: len
        }

    })))                

}*/

//获取分类列表
//get /api/admin/categories
pub async fn categories_handler(
    app_state: web::Data<AppState>,//把服务器当作参数传入
    ) -> Result<HttpResponse, MyError> {
    let categories = get_admin_categories(&app_state.db).await?;

    println!("new request!");

    
    // let data: Vec<response_state::CategoriesResponse> = categories.iter()
    //                      .map(|category| 
    //                           response_state::CategoriesResponse {
    //                                 id: category.0 as i32,
    //                                 name: category.1.clone(),
    //                                 amount:12
    //                      })
    //                     .collect();

     
    Ok(HttpResponse::Ok().json(json!({
        "code":200,
        "data": categories
    })))
}

//获取所有评论审核
//get /api/admin/comments
/*pub async fn comments_handler(
    app_state: web::Data<AppState>,//把服务器当作参数传入
    ) -> Result<HttpResponse, MyError> {
    let comments = get_admin_comments(&app_state.db).await;
    println!("new request!");

    Ok(HttpResponse::Ok().json(json!({
        "code":200,
        "data":comments.iter()
                    .map(|comment| response_state::Comments {
                            id: comment.0,
                            articles: response_state::CommentArticle {
                                id: comment.1,
                                title: comment.2.clone()
                            },
                            user: response_state::CommentUser{
                                nickname: comment.3.clone()
                            },
                            content: comment.4.clone(),
                            status: comment.5,
                            createAt: comment.6.clone()
                    })
                    .collect::<Vec<response_state::Comments>>()
    })))
}*/

//获取标签列表
//get /api/admin/tags
pub async fn tags_handler(
    app_state: web::Data<AppState>,//把服务器当作参数传入
    ) -> Result<HttpResponse, MyError> {
    let tags = get_admin_tags(&app_state.db).await?;
    println!("new request!");

    Ok(HttpResponse::Ok().json(json!({
        "code":200,
        "data":tags
    })))
}

//删除文章
//post /api/admin/deleteArticle
/*pub async fn delect_article_handler(
    app_state: web::Data<AppState>,//把服务器当作参数传入
    delete_info: web::Json<request_state::DeleteDraft>
    ) -> Result<HttpResponse, MyError> {

    println!("new request! from deleteArticle");
    let delete = post_admin_deleteArticle(&app_state.db, delete_info.id).await;

    Ok(HttpResponse::Ok().json(json!({
        "code":200,
        "data":response_state::DeleteDraftResponse {
                id: delete
        }
    })))
}*/


//新增标签
//post /api/admin/createAt
/*pub async fn create_tag_handler(
    app_state: web::Data<AppState>,//把服务器当作参数传入
    info: web::Json<request_state::createTagInfo>
    ) -> HttpResponse {

    println!("new request! in add new tag");
    let tags = post_admin_createTag(&app_state.db, info.name.clone()).await;
    println!("new request! in add new tag");
    HttpResponse::Ok().json(json!({
        "code":200,
        "data": response_state::TagsListResponse {
                id: tags.0,
                name: info.name.clone(),
                createAt: tags.1.clone()
        }
    }))
}*/

//更改分类
pub async fn put_categories_handler(
    app_state: web::Data<AppState>,//把服务器当作参数传入
    category_info: web::Json<request_state::CategoryInfo>
    ) -> Result<HttpResponse, MyError> {
    update_admin_category(&app_state.db, 
                          category_info.category_id, 
                          category_info.name.clone()).await?;

    println!("new request!");

    
    // let data: Vec<response_state::CategoriesResponse> = categories.iter()
    //                      .map(|category| 
    //                           response_state::CategoriesResponse {
    //                                 id: category.0 as i32,
    //                                 name: category.1.clone(),
    //                                 amount:12
    //                      })
    //                     .collect();

     
    Ok(HttpResponse::Ok().json(json!({
        "code":200,
        "msg": "修改成功"
    })))
}

//删除分类
pub async fn del_categories_handler(
    app_state: web::Data<AppState>,//把服务器当作参数传入
    category_info: web::Query<request_state::CategoryInfo>
    ) -> Result<HttpResponse, MyError> {
    del_admin_category(&app_state.db, category_info.category_id).await?;

    println!("new request!");

    
    // let data: Vec<response_state::CategoriesResponse> = categories.iter()
    //                      .map(|category| 
    //                           response_state::CategoriesResponse {
    //                                 id: category.0 as i32,
    //                                 name: category.1.clone(),
    //                                 amount:12
    //                      })
    //                     .collect();

     
    Ok(HttpResponse::Ok().json(json!({
        "code":200,
        "msg": "删除成功"
    })))
}

//添加分类
pub async fn add_categories_handler(
    app_state: web::Data<AppState>,//把服务器当作参数传入
    category_info: web::Json<request_state::CategoryInfo>
    ) -> Result<HttpResponse, MyError> {
    add_admin_category(
        &app_state.db, category_info.name.clone()).await?;

    println!("new request!");

    
    // let data: Vec<response_state::CategoriesResponse> = categories.iter()
    //                      .map(|category| 
    //                           response_state::CategoriesResponse {
    //                                 id: category.0 as i32,
    //                                 name: category.1.clone(),
    //                                 amount:12
    //                      })
    //                     .collect();

     
    Ok(HttpResponse::Ok().json(json!({
        "code":200,
        "msg": "ok"
    })))
}
