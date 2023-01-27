use actix_web::{web, App, HttpResponse};
use crate::errors::MyError;
use crate::models::{descriptor::*, markdown::*};
use crate::state::{app_state::*, request_state, response_state};
use serde_json::json;
use crate::db::drafts::*;

//保存为草稿
//post /api/write/saveDraft
/*pub async fn save_draft_handler(
    app_state: web::Data<AppState>,//把服务器当作参数传入
    info: web::Json<request_state::SaveDraftInfo>
    ) -> HttpResponse {
    
    println!("new request! save draft handler");
    let draft = post_write_savedraft(
                                        &app_state.db,
                                        info.markdown.clone(),
                                        info.title.clone()).await;
    println!("new request! save draft handler");

    match draft {
        true => {
            HttpResponse::Ok().json(json!({
                "code": 200,
                "msg": "保存草稿成功"
            }))
        },
        false => {
            HttpResponse::Ok().json(json!({
                "code": 400,
                "msg": "保存为草稿失败"
            }))
        }
    }
}*/

//获取所有草稿文章
//get /api/write/drafts
pub async fn drafts_handler(
    app_state: web::Data<AppState>,//把服务器当作参数传入
    ) -> Result<HttpResponse, MyError> {

    println!("new request!");

    get_write_drafts(&app_state.db)
        .await
        .map(|drafts| HttpResponse::Ok().json(json!({
            "code": 200,
            "data": drafts
        })))

}


//获得草稿箱中的一篇文章
//get /api/write/draft
pub async fn draft_handler(
    app_state: web::Data<AppState>,//把服务器当作参数传入
    request: web::Query<request_state::DraftInfo>
    ) -> Result<HttpResponse, MyError> {
    println!("new request!");
    get_write_draft(&app_state.db,
                    request.article_id)
        .await
        .map(|draft| HttpResponse::Ok().json(json!({
            "code":200,
            "data": draft
        })))
/*    let response = response_state::DraftResponse {
        markdown: draft.1,
        title: draft.0
    };
    HttpResponse::Ok().json(json!({
        "code":200,
        "data": response
    }))*/
}


//对原草稿箱文章进行更新
//post /api/write/updateDraft
/*pub async fn update_draft_handler(
    app_state: web::Data<AppState>,//把服务器当作参数传入
    request: web::Json<request_state::UpdateDraft>
    ) -> HttpResponse {
    
    let row = post_write_updatedraft(
                                        &app_state.db,
                                        request.id,
                                        request.title.clone(),
                                        request.markdown.clone()).await;
    println!("new request!");
    match row {
        true => {
            HttpResponse::Ok().json(json!({
                "code": 200,
                "msg": "保存草稿成功"
            }))
        },
        false => {
            HttpResponse::Ok().json(json!({
                "code": 400,
                "msg": "保存为草稿失败"
            }))
        }
    }
}*/


//删除草稿文章
//post /api/write/deleteDraft
pub async fn delete_draft_handler(
    app_state: web::Data<AppState>,//把服务器当作参数传入
    request: web::Json<request_state::DraftInfo>
    ) -> Result<HttpResponse, MyError> {

    println!("new request!");

    post_write_deletedraft(&app_state.db,request.article_id).await?;

    Ok(HttpResponse::Ok().json(json!({
        "code":200,
        "msg": "删除成功"
        }
    )))
}

//发布新文章
//post /api/write/publish
pub async fn publish_handler(
    app_state: web::Data<AppState>,//把服务器当作参数传入
    request: web::Json<request_state::Publish>
    ) -> Result<HttpResponse, MyError> {
    
    println!("new request");
    post_write_publish(&app_state.db,
                       request.content.clone(),
                       request.tags.clone(),
                       request.category).await?;
    
    Ok(HttpResponse::Ok().json(json!({
        "code": 200,
        "msg": "上传文章成功"
    })))
}
