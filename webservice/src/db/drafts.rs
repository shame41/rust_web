use crate::{models::user::*, state::response_state, errors::MyError};
use sqlx::{postgres::PgPool, Executor};

/*pub async fn post_write_savedraft(pool: &PgPool, push_markdown: String, push_title: String) -> bool {
    let user = sqlx::query!(
        r#"INSERT INTO articles (title, markdown, view_num, favorite_num, comment_num, ispublish, category)
        VALUES ($1, $2, $3, $4, $5, $6, $7)"#,
        push_title,
        push_markdown,
        0,
        0,
        0,
        false,
        "DEFAULT".to_string()
        )
        .execute(pool)
        .await;

    true 
}*/

pub async fn get_write_drafts(pool: &PgPool) 
    -> Result<Vec<response_state::Draft>, MyError> {
/*    let users = sqlx::query!(
        r#"SELECT id, title, ispublish, article_time
        FROM articles "#,
        )
        .fetch_all(pool)
        .await?;*/

    let drafts = vec![response_state::Draft{
            article_id: 1,
            content: "content".to_string(),
            is_publish: false,
            updatad_at: "time".to_string()
    }];
    
    Ok(drafts)
}

pub async fn get_write_draft(pool: &PgPool, id: i32) 
    -> Result<response_state::DraftResponse, MyError> {
/*    let user = sqlx::query!(
        r#"SELECT title, markdown
        FROM articles 
        WHERE articles.id = $1"#,
        id
        )
        .fetch_one(pool)
        .await?;*/

        Ok(response_state::DraftResponse{
            content: "content".to_string(),
            article_id: 1
        })
}

/*pub async fn post_write_updatedraft
        (pool: &PgPool, id: i32, push_title: String, push_markdown: String) 
            -> bool {
    let user = sqlx::query!(
        r#"UPDATE articles
        SET markdown = $1,
            title = $2
        WHERE articles.id = id"#,
        push_markdown,
        push_title
        )
        .execute(pool)
        .await;

    true
}*/


pub async fn post_write_deletedraft(pool: &PgPool, id: i32) 
    -> Result<(), MyError> {
/*    let user = sqlx::query!(
        r#"DELETE FROM articles
        WHERE id = $1"#,
        id
        )
        .fetch_one(pool)
        .await?;*/

        Ok(())
}

/*pub async fn post_write_publish
    (pool: &PgPool,id: Option<i32>, push_markdown: String, push_title: String, 
            articleTags: Vec<String>, articleCategory: String)
    -> bool {
    
       let publish_article = match id {
            Some(id) => {
                println!("Some");
                for tag in articleTags {
                    //先查询tags
                    let select_tags = sqlx::query!(
                                        r#"SELECT tags.id 
                                        FROM tags
                                        WHERE tagname = $1"#,
                                        tag.clone()
                                        ).fetch_one(pool).await.unwrap();
                        println!("got tags");
                        println!("{}", select_tags.id);
                    //然后更新tags_articles
                        sqlx::query!(
                            r#"INSERT INTO tags_articles
                            (article_id, tag_id)
                            VALUES ($1, $2)"#,
                            id,
                            select_tags.id
                            ).execute(pool).await.unwrap();
                        println!("insert ok {}", tag.clone());
                }
                println!("OK");
                //更新文章并公开
                sqlx::query!(
                r#"UPDATE articles
                SET ispublish = $1,
                category = $2,
                markdown = $3,
                title = $4
                WHERE articles.id = $5"#,
                true,
                articleCategory,
                push_markdown,
                push_title,
                id
                ).execute(pool).await.unwrap();
 
            }
            None => {
                println!("None");
                let insert = sqlx::query!(
                r#"INSERT INTO articles 
                (title, markdown, view_num, favorite_num, comment_num, ispublish, category)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                RETURNING id"#,
                push_title,
                push_markdown,
                0,
                0,
                0,
                true,
                articleCategory,
                )
                .fetch_one(pool).await.unwrap();

                for tag in articleTags{
                    //先查询tags
                    let select_tags = sqlx::query!(
                                r#"SELECT tags.id 
                                FROM tags
                                WHERE tagname = $1"#,
                            tag.clone()
                        ).fetch_one(pool).await.unwrap();
                    //然后更新或者插入新tag
                                    sqlx::query!(
                                    r#"INSERT INTO tags_articles
                                    (article_id, tag_id)
                                    VALUES ($1, $2)"#,
                                    insert.id,
                                    select_tags.id 
                                    ).execute(pool).await.unwrap();
                }
            }
       };
    true 
                             
}*/

pub async fn post_write_publish(pool: &PgPool, content: String,
                                tags_id: Vec<i32>, category_id: i32)
    -> Result<(), MyError> {

    Ok(())
}
