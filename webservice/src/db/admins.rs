use crate::{models::user::*, state::response_state, errors::MyError};
use sqlx::{postgres::PgPool, Executor};

/*pub async fn get_admin_articles(pool: &PgPool, page: i32, pageSize: i32) 
        -> Result<response_state::Article, MyError> {
    let row = sqlx::query!(
        r#"SELECT id, title, category, favorite_num, view_num, comment_num, article_time
        FROM articles
        WHERE ispublish = $1
        LIMIT $2
        OFFSET $3"#,
        true,
        pageSize as i64,
        (pageSize * (page - 1)) as i64
        )
        .fetch_all(pool)
        .await?;

/*    row.iter()
        .map(|r| (r.id, r.title.clone(), r.category.clone(), r.favorite_num, r.view_num, r.comment_num, r.article_time.to_string()))
        .collect();*/
    let article_vec = vec![response_state::AdminArticles{
        id: 1,
        title

    }];

    Ok(response_state::Article{
        articles: article_vec,
        article_count: 12,
    })
    
}*/


pub async fn get_admin_categories(pool: &PgPool) 
        -> Result<response_state::Category, MyError> {
/*    let row = sqlx::query!(
        r#"SELECT category
        FROM articles"#
        )
        .fetch_all(pool)
        .await?;*/

    Ok(response_state::Category{
        category_id: 1,
        name: "name".to_string(),
        amount: 12,
        
    })    
}

/*pub async fn get_admin_comments(pool: &PgPool) 
        -> Vec<(i32, i32, String, String, String, bool, String)> {
    let row = sqlx::query!(
        r#"SELECT comments.id, article_id, title, username, content, comment_status, comment_time
        FROM comments join articles
        ON comments.article_id = articles.id
        JOIN users 
        ON comments.user_id = users.id"#
        )
        .fetch_all(pool)
        .await
        .unwrap();

    row.iter()
        .map(|r| (r.id, r.article_id, r.title.clone(), r.username.clone(),
                  r.content.clone(), r.comment_status, r.comment_time.to_string()))
        .collect()
    
}*/


pub async fn get_admin_tags(pool: &PgPool) 
        -> Result<Vec<response_state::AdminTag>, MyError> {
/*    let row = sqlx::query!(
        r#"SELECT id, tagname, tag_time
        FROM tags"#
        )
        .fetch_all(pool)
        .await?;*/
    let tags = vec![response_state::AdminTag{
        id: 1,
        name: "name".to_string(),
        create_at: "time".to_string()
    }];
    Ok(tags)
}

pub async fn del_admin_category(pool: &PgPool, category_id: i32) 
        -> Result<(), MyError> {
/*    let row = sqlx::query!(
        r#"SELECT id, tagname, tag_time
        FROM tags"#
        )
        .fetch_all(pool)
        .await?;*/

    Ok(())   
}

pub async fn update_admin_category(pool: &PgPool, category_id: i32, name: String) 
        -> Result<(), MyError> {
/*    let row = sqlx::query!(
        r#"SELECT id, tagname, tag_time
        FROM tags"#
        )
        .fetch_all(pool)
        .await?;*/

    Ok(())   
}

pub async fn add_admin_category(pool: &PgPool, name: String) 
        -> Result<(), MyError> {
/*    let row = sqlx::query!(
        r#"SELECT id, tagname, tag_time
        FROM tags"#
        )
        .fetch_all(pool)
        .await?;*/

    Ok(())   
}
/*pub async fn post_admin_deleteArticle(pool: &PgPool, article_id: i32) 
        -> i32 {
        sqlx::query!(
        r#"DELETE
        FROM articles
        WHERE articles.id = $1"#,
        article_id
        )
        .execute(pool)
        .await
        .unwrap();

        sqlx::query!(
        r#"DELETE
        FROM tags_articles
        WHERE article_id = $1"#,
        article_id
        )
        .execute(pool)
        .await
        .unwrap();

    article_id    
}*/

/*pub async fn post_admin_createTag(pool: &PgPool, tag_name: String) -> (i32, String) {
        sqlx::query!(
        r#"INSERT INTO tags
        (tagname)
        VALUES ($1)"#,
        tag_name
        )
        .execute(pool)
        .await
        .unwrap();
    
    let new_tag = sqlx::query!(
        r#"SELECT id, tag_time
        FROM tags
        WHERE tagname = $1"#,
        tag_name
        )
        .fetch_one(pool)
        .await
        .unwrap();

    (new_tag.id, new_tag.tag_time.to_string())
}*/


