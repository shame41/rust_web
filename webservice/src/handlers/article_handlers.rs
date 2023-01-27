use actix_web::{web, App, HttpResponse};
use crate::errors::MyError;
use crate::state::{app_state::*, request_state, response_state};
use serde_json::json;
use crate::db::articles::*;

//获取主页上的一页文章列表
//get /api/article/articles
pub async fn articles_handler(
    app_state: web::Data<AppState>,//把服务器当作参数传入
    artical_info: web::Json<request_state::ArticalInfo>
    ) -> Result<HttpResponse, MyError> {

    println!("new request for articles");
   
    let article = get_article_articles(&app_state.db,
                         artical_info.current_page,
                         artical_info.page_size)
        .await?;
/*    let temp: Vec<D> = get_article_articles(
                    &app_state.db,
                    artical_info.currentPage,
                    artical_info.pageSize)
                    .await?
                    .map(|a| response_state::Descriptor {
                        view: a.view,
                        category: a.category.clone(),
                        favorite: a.favorite,
                        comment: a.comment,
                        tags: a.tags.clone(),
                        create_at: a.create_at.clone(),
                        article_id: a.article_id
                    }).collect();
    
    /* let temp : Vec<response_state::Descriptor> = articles.iter()
                .map(|a| response_state::Descriptor {
                    view: a.0,
                    category: a.1.clone(),
                    favorite: a.2,
                    comment: a.3,
                    title: a.4.clone(),
                    tags: a.5.clone(),
                    createAt: a.6.clone(),
                    articleId: a.7
                }).collect();*/

    let len = articles.len();
    /*let response = response_state::ArticalsResponse {
        articles: temp,
        article_count: len as i32
    };*/*/
    
    let len = article.len();
    Ok(HttpResponse::Ok().json(json!({
        "code": 200,
        "data": response_state::ArticalsResponse {
            articles: article,
            article_count: len as i32
        }
    })))
}

//进入文章详情页面
//get /api/article/detail
pub async fn detail_handler(
    app_state: web::Data<AppState>,//把服务器当作参数传入
    artical_info: web::Query<request_state::ArticalDetail>
    ) -> Result<HttpResponse, MyError> { 
    let articles = get_article_detail(
                                        &app_state.db,
                                        artical_info.article_id).await?;
    println!("new request!");


    // let response = response_state::DetailResponse {
    //     view: articles.0,
    //     content: articles.1,
    //     author_id: articles.2,
    //     create_at: vec!["1".to_string()],
    //     comment: articles.4,
    //     is_favorite: true,
    //     favorite: articles.5
    // };

    Ok(HttpResponse::Ok().json(json!({
        "code": 200,
        "data": articles
    })))
}

//获取文章下已经发表的评论内容
//get /api/article/comments
/*pub async fn comment_handler(
    app_state: web::Data<AppState>,//把服务器当作参数传入
    article_info: web::Query<request_state::CommentInfo>
    ) -> Result<HttpResponse, MyError> {
    let comments = get_article_comments(
                                        &app_state.db,
                                        article_info.articleId).await;
    println!("new request!");

    let response = response_state::CommentResponse {
        code: 200, 
        data: comments.iter()
            .map(|comment| response_state::Comment {
                author: response_state::User { 
                    nickname: comment.0.clone(),
                    avatar: comment.1.clone() 
                },
                content: comment.2.clone(),
                createAt: comment.3.clone()
            })
            .collect()
    };
    HttpResponse::Ok().json(response)
}*/

//获取所有已经存在的标签
//get /api/article/tags
pub async fn tags_handler(
    app_state: web::Data<AppState>,//把服务器当作参数传入
    ) -> Result<HttpResponse, MyError> {
    let tags = get_article_tags(&app_state.db).await?;
    println!("new request tags!");

    Ok(HttpResponse::Ok().json(json!({
        "code": 200,
        "data": tags 
    })))
}

//文章点赞状态变化
//post /api/article/favorite
/*pub async fn favorite_handler(
    app_state: web::Data<AppState>,//把服务器当作参数传入
    request: web::Json<request_state::FavoriteInfo>
    ) -> Result<HttpResponse, MyError> {

    
    let favorite = post_article_favorite(
                                            &app_state.db,
                                            request.articleId,
                                            request.userId).await;
    println!("new request!: from favor!");

    HttpResponse::Ok().json(json!({
        "code": 200,
        "msg": "Ok"
    }))
}*/

//文章是否被点赞过
//get /api/article/isFavorite
/*pub async fn isfavorite_handler(
    app_state: web::Data<AppState>,//把服务器当作参数传入
    is_favorite: web::Query<request_state::FavoriteInfo>
    ) -> Result<HttpResponse, MyError> {
    let isfavorite = get_article_isfavorite(
                                            &app_state.db,
                                            is_favorite.articleId,
                                            is_favorite.userId).await;
    println!("new request! : is favor?");

    HttpResponse::Ok().json(json!({
        "code":200,
        "data":isfavorite as i64
    }))
}*/

//已经登陆的用户添加文章评论
//post /api/article/addComment
/*pub async fn addcomment_handler(
    app_state: web::Data<AppState>,//把服务器当作参数传入
    info: web::Json<request_state::AddComment>
    ) -> Result<HttpResponse, MyError> {
     
    let comment = post_article_addcomment(
                                            &app_state.db,
                                            info.content.clone(),
                                            info.articleId,
                                            info.author).await;
    println!("new request!");
    let response = response_state::Comment {
        author: response_state::User {
            nickname: comment.0,
            avatar: comment.1
        },
        content: comment.2,
        createAt: comment.3,
    };
    HttpResponse::Ok().json(json!({
        "code":200,
        "data": response
    }))
}*/

//获得所有分类以及对应文章数量
//get /api/article/classes
pub async fn classes(
    app_state: web::Data<AppState>,//把服务器当作参数传入
    ) -> Result<HttpResponse, MyError> {
    let cats = get_article_classes(&app_state.db).await?;
    println!("new request!");
    // let class: Vec<response_state::Class> = cats.iter()
    //     .map(|cat| response_state::CategoriesNum {
    //             name: cat.0.clone(),
    //             number: *cat.1
    //     }).collect();
    // println!("{}", data.len());
    Ok(HttpResponse::Ok().json(json!({
        "code": 200,
        "data": cats
    })))
}

/*pub async fn getByClass_handler(
    app_state: web::Data<AppState>,//把服务器当作参数传入
    class_info: web::Query<request_state::ArticalByClass>
    ) -> Result<HttpResponse, MyError> {

    println!("new request for articles");
    let articles = get_article_articles( &app_state.db,
                                           class_info.currentPage,
                                           class_info.pageSize,
                                           class_info.class1.clone()).await;

    println!("new request!");


    let temp : Vec<response_state::Descriptor> = articles.iter()
                .map(|a| response_state::Descriptor {
                    view: a.0,
                    category: a.1.clone(),
                    favorite: a.2,
                    comment: a.3,
                    title: a.4.clone(),
                    tags: a.5.clone(),
                    createAt: a.6.clone(),
                    articleId: a.7
                }).collect();
    let len = temp.len();

    HttpResponse::Ok().json(json!({
        "code": 200,
        "data": response_state::Articles{
            articles: temp 
        },
        "articleCount": len
    }))
}

pub async fn getByTag_handler(
    app_state: web::Data<AppState>,//把服务器当作参数传入
    tag_info: web::Query<request_state::ArticalByTag>
    ) -> Result<HttpResponse, MyError> {

    println!("new request for articles");
    let articles = get_article_articles( &app_state.db,
                                           tag_info.currentPage,
                                           tag_info.pageSize,
                                           tag_info.tag.clone()).await;

    println!("new request!");


    let temp : Vec<response_state::Descriptor> = articles.iter()
                .map(|a| response_state::Descriptor {
                    view: a.0,
                    category: a.1.clone(),
                    favorite: a.2,
                    comment: a.3,
                    title: a.4.clone(),
                    tags: a.5.clone(),
                    createAt: a.6.clone(),
                    articleId: a.7
                }).collect();
    let len = temp.len();
    
    HttpResponse::Ok().json(json!({
        "code": 200,
        "data": response_state::Articles {
                articles: temp
        },
        "articleCount": len
    }))
}*/

