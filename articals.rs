use crate::models::descriptor::*;
use crate::models::markdown::*;
use sqlx::{postgres::PgPool, Executor};

pub async fn select_articals_by_page(pool: &PgPool, current_page: i32, page_size: i32) -> Vec<Descriptor> {

    let rows = sqlx::query!(
        r#"SELECT title,id,time,descriptor,favorite, 
        FROM markdowns
        WHERE email = $1
        AND password = $2"#,
        current_page,
        page_size
        )
        .fetch_all(pool)
        .await
        .unwrap();
    
    rows.iter()
        .map(|row| Descriptor {
            id: row.id,
            title: row.title.clone(),
            view: 1,
            comment: row.comment,
            time: row.time.clone(),
            category: row.category.clone(),
            tags: vec!["1".to_string(),"2".to_string()]
        })
        .collect()
}

pub async fn select_artical_detail_by_id(pool: &PgPool, page_id: i32) -> Markdown {

    let row = sqlx::query!(
        r#"SELECT title,id, category, view, time,favorite, comment,
        FROM markdowns
        WHERE page_id = $1"#,
        page_id
        )
        .fetch_one(pool)
        .await
        .unwrap();
    
    Markdown {
        id: row.id,
        title: row.title,
        view: row.view,
        category: row.category,
        time: row.time,
        comment: row.comment,
        isPublish: true,
        tag1: true,
        tag2: true,
        tag3: true,
        tag4: true
    }
}
