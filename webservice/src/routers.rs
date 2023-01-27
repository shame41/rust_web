use crate::handlers::{first_handler::*, user_handlers, article_handlers, draft_handlers, admin_handlers};
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(first_handler));
}

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/user")
                    .route("/login", web::post().to(user_handlers::login_handler))
                    .route("/account", web::get().to(user_handlers::get_account_handler))
                    // .route("/register", web::post().to(user_handlers::register_handler))
                    .route("/logout", web::post().to(user_handlers::logout_handler))
                );
}

pub fn article_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/article")
                    .route("/articles", web::post().to(article_handlers::articles_handler))
                    .route("/detail", web::get().to(article_handlers::detail_handler))
                    // .route("/comments", web::get().to(article_handlers::comment_handler))
                    .route("/tags", web::get().to(article_handlers::tags_handler))
                    // .route("/favorite", web::post().to(article_handlers::favorite_handler))
                    // .route("/isFavorite", web::get().to(article_handlers::isfavorite_handler))
                    // .route("/addComment", web::post().to(article_handlers::addcomment_handler))
                    .route("/classes", web::get().to(article_handlers::classes))
                );
}

pub fn write_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/write")
                    // .route("/saveDraft", web::post().to(draft_handlers::save_draft_handler))
                    .route("/drafts", web::get().to(draft_handlers::drafts_handler))
                    .route("/draft", web::get().to(draft_handlers::draft_handler))
                    // .route("/updateDraft", web::get().to(draft_handlers::update_draft_handler))
                    .route("/deleteDraft", web::post().to(draft_handlers::delete_draft_handler))
                    .route("/publish", web::post().to(draft_handlers::publish_handler))
                );
}

pub fn admin_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/admin")
                    // .route("/articles", web::get().to(admin_handlers::articles_handler))
                    .route("/getCategories", web::get().to(admin_handlers::categories_handler))
                    // .route("/comments", web::get().to(admin_handlers::comments_handler))
                    .route("/tags", web::get().to(admin_handlers::tags_handler))
                    // .route("/deleteArticle", web::post().to(admin_handlers::delect_article_handler))
                    // .route("/createTag", web::post().to(admin_handlers::create_tag_handler))
                    .route("/putCategorys", web::put().to(admin_handlers::put_categories_handler))
                    .route("/deleteCategory", web::delete().to(admin_handlers::del_categories_handler))
                    .route("/addCategory", web::post().to(admin_handlers::add_categories_handler))
                );
}
