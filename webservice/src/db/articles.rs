use std::collections::HashMap;
use crate::state::response_state;
use crate::{models::user::*, errors::MyError};
use sqlx::{postgres::PgPool, Executor};

pub async fn get_article_articles(pool: &PgPool, current_page: i32, page_size: i32) 
    // -> Result<Vec<(i32, String, i32, i32, Vec<String>, String, i32)>, MyError> {
    ->Result<Vec<response_state::Article>, MyError> {
/*    let rows = sqlx::query!(
        r#"SELECT view_num, category, favorite_num, comment_num, tagname, article_time, articles.id 
        from articles 
        join tags_articles
        on articles.id = tags_articles.article_id
        join tags
        on tags.id = tags_articles.tag_id
        LIMIT $1
        OFFSET $2"#,
        page_size as i64,
        (page_size *(current_page - 1)) as i64
        )
        .fetch_all(pool)
        .await?;*/

    let articles = vec![response_state::Article{
        view: 1,
        category: "category".to_string(),
        favorite: 1,
        comment: 1,
        tags: Vec::<String>::new(),
        create_at: "time".to_string(),
        article_id: 1
    }];
    Ok(articles)

/*
    let mut article = rows.iter() 
        .map(|r| (r.view_num, r.category.clone(), r.favorite_num, r.comment_num,
                r.tagname.clone(), r.article_time.to_string(), r.id))
        .collect::<Vec<(i32, String, i32, i32, String, String, i32)>>();
    println!("get row.iter() {}", article.len());
    let mut first_article = article.remove(0); 
    let mut list: Vec<Descriptor> = 
                vec![Descriptor{
                    view: first_article.0,
                    category: first_article.1.clone(),
                    favorite: first_article.2,
                    comment: first_article.3,
                    tags: Vec::new(),
                    create_at: first_article.5,
                    article_id: first_article.6
                }];

    for a in article {
       if a.6 == first_article.6 {
            println!("{} == {}", a.6, first_article.6);
            let mut temp = list.pop().unwrap();//取出list的第一个元素
            temp.4.push(a.4);
            list.push(temp);
            println!("if {}", list.len());
       }else{
           list.push((a.0, a.1.clone(), a.2, a.3, vec![a.4.clone()], a.5.clone(), a.6));
           first_article = a;
           println!("else {}", list.len());
       }
    };
    println!("len of list {}", list.len());
    Ok(list)
    */
}

/*pub async fn get_article_articles(pool: &PgPool, current_page: i32, page_size: i32) 
    ->Result<Vec<Descriptor>, MyError> {
    println!("in get_article_articles 1");
    let rows = sqlx::query!(
        r#"SELECT view_num, category, favorite_num, comment_num, tagname, article_time, articles.id 
        from articles 
        join tags_articles
        on articles.id = tags_articles.article_id
        join tags
        on tags.id = tags_articles.tag_id
        LIMIT $1
        OFFSET $2"#,
        page_size as i64,
        (page_size *(current_page - 1)) as i64
        )
        .fetch_all(pool)
        .await?;

    //第一步，获得将rows中的id和tag存到一个map中，其中tag用vec来存，并且初始时tag为空，id是Key
    //第二步，用这个map遍历整个rows，根据id来递增tags
    //第三步，根据id将rows中的结果去重，并存入descriptor数组中
    //第四步，用得到的map遍历descriptor数组，更新它的tags
    let descriptors: Vec<Descriptor> = rows.iter()
        .map(|row| Descriptor {
            view: row.view_num,
            category: row.category,
            favorite: row.favorite_num,
            comment: row.comment_num,
            tags: Vec::new(),
            create_at: row.article_time,
            article_id: row.id
        }).collect();
    let da = descriptors.iter();
    let db = descriptors.iter().skip(1);
    da.filter(|d| d.article_id == db.next().unwrap().article_id)

    let tags_map: HashMap<int, Vec<String>> = HashMap::new();
    for row in rows {
        if !tags_map.contains_key(row.id) {
            tags_map.insert(row.id, Vec::new());
        }
    }
    for row in rows {
        tags_map.get_mut(row.id).unwrap().push(row.tagname);
    }

    let descriptors :Vec<Descriptor> = tags_map.iter().map(|t| Descriptor{
         
    });
    


    let mut article = rows.iter()
        .map(|r| (r.view_num, r.category.clone(), r.favorite_num, r.comment_num,
                r.tagname.clone(), r.article_time.to_string(), r.id))
        .collect::<Vec<(i32, String, i32, i32, String, String, String, i32)>>();
    println!("get row.iter() {}", article.len());
    let mut first_article = article.remove(0); 
    let mut list: Vec<(i32, String, i32, i32, String, Vec<String>, String, i32)> = 
                vec![(first_article.0, first_article.1.clone(), first_article.2,
                    first_article.3, first_article.4.clone(), vec![first_article.5].clone(), 
                    first_article.6.clone(),
                    first_article.7)];
    for a in article {
       if a.4.clone() == first_article.4.clone() {
            println!("{} == {}", a.4.clone(), first_article.4.clone());
            let mut temp = list.pop().unwrap();
            temp.5.push(a.5);
            list.push(temp);
            println!("if {}", list.len());
       }else{
           list.push((a.0, a.1.clone(), a.2, a.3, a.4.clone(), vec![a.5.clone()], a.6.clone(), a.7));
           first_article = a;
           println!("else {}", list.len());
       }
    };
    println!("len of list {}", list.len());
    Ok(list)

}*/
pub async fn get_article_detail(pool: &PgPool, id: i32) 
        -> Result<response_state::DetailResponse, MyError> {
/*    let user = sqlx::query!(
        r#"SELECT read_num, content, author_id, comment_num, like_num, create_at
        FROM articles
        WHERE id = $1"#,
        id
        )
        .fetch_one(pool)
        .await?;*/

    Ok(response_state::DetailResponse{
        view: 1,
        content: "content".to_string(),
        author_id:1,
        create_at: "time".to_string(),
        comment: 1,
        is_favorite: true,
        favorite: 1,
    })
}

/*pub async fn get_article_comments(pool: &PgPool, id: i32) -> Vec<(String, String, String, String)> {
    let user = sqlx::query!(
        r#"SELECT username,avatar,content, comment_time 
        FROM articles join comments 
        on articles.id = comments.article_id 
        join users 
        on users.id = comments.user_id
        WHERE articles.id = $1"#,
        id
        )
        .fetch_all(pool)
        .await
        .unwrap();
    
    user.iter()
        .map(|r| (r.username.clone(), r.avatar.clone(), 
                  r.content.clone(), r.comment_time.clone().to_string()))
        .collect()
}*/

pub async fn get_article_tags(pool: &PgPool) 
    -> Result<Vec<response_state::Tag>, MyError> {
/*    let user = sqlx::query!(
        r#"SELECT id, name
        FROM tags"#,
        )
        .fetch_all(pool)
        .await?;*/
    // let tags_vec = user.iter()
    //     .map(|r| response_state::Tag{
    //         name: "name".to_string(),
    //         tag_id: 1 
    //     })
    //     .collect();
    let tags_vec = vec![response_state::Tag{
        name: "name".to_string(),
        tag_id: 1
    }];
    Ok(tags_vec)
}

/*pub async fn post_article_favorite(pool: &PgPool, articleid: i32, userid: i32) -> bool {

    let favorite = sqlx::query!(
        r#"select favorite_num
        from articles
        where id = $1"#,
        articleid
        )
        .fetch_one(pool)
        .await
        .unwrap();
    println!("{}", favorite.favorite_num);

    let user = sqlx::query!(
        r#"update articles
        set favorite_num = $1
        where id = $2"#,
        1 as i32,
        articleid
        )
        .execute(pool)
        .await;

    println!("favor"); 
    let user = sqlx::query!(
        r#"insert into favorite
        (article_id, user_id, favorite_status)
        values($1, $2, $3)"#,
        articleid,
        userid,
        true,
       )
       .execute(pool)
       .await;
    
        true
}*/

/*pub async fn get_article_isfavorite(pool: &PgPool, articleid: i32, userid: i32) -> bool {
    let user = sqlx::query!(
        r#"SELECT favorite_status
        from articles 
        join favorite
        on articles.id = favorite.article_id
        join users
        on users.id = user_id
        where articles.id = $1
        and users.id = $2"#,
        articleid,
        userid
        )
        .fetch_optional(pool)
        .await
        .unwrap();
    match user {
        Some(u) => true,
        None => false
    }
}*/

/*pub async fn post_article_addcomment
        (pool: &PgPool, content: String, articleid: i32, userid: i32) 
        -> (String, String, String, String) {
    let user = sqlx::query!(
        r#"INSERT INTO comments (comment_status, content, article_id, user_id)
        VALUES ($1, $2, $3, $4)"#,
        false,
        content,
        articleid,
        userid
        )
        .execute(pool)
        .await
        .unwrap();

    let user = sqlx::query!(
        r#"SELECT username, avatar, content, comment_time
        FROM comments JOIN users
        ON users.id = comments.user_id
        JOIN articles
        ON articles.id = comments.article_id
        WHERE users.id = $1
        AND articles.id = $2"#,
        userid,
        articleid
        )
        .fetch_one(pool)
        .await
        .unwrap();
    
    (user.username.clone(), user.avatar.clone(), 
     user.content.clone(), user.comment_time.clone().to_string())
}*/

pub async fn get_article_classes(pool: &PgPool) 
    -> Result<Vec<response_state::Category>, MyError> {
/*    let user = sqlx::query!(
        r#"select id, name, count(articles.id) as amount
        from categoris
        join articles on categoris.id = articles.category_id
        group by name"#,
        )
        .fetch_all(pool)
        .await?;*/

/*    user.iter()
        .map(|u| {
        println!("now category is {}", u.category);
        if counter.contains_key(&u.category) {
            let temp: i32 = *counter.get(&u.category).unwrap() + 1;
            println!("now num is{}", temp);
            counter.entry(u.category.clone()).or_insert(temp);
        }else {
                
                counter.insert(u.category.clone(), 1_i32);
            }
        });*/
    let categories = vec![response_state::Category{
        category_id: 1,
        name: "name".to_string(),
        amount: 1 
    }];
    Ok(categories)    
}


/*pub async fn get_article_byClass(pool: &PgPool, current_page: i32, page_size: i32, class: String) -> Vec<(i32, String, i32, i32, String, Vec<String>, String, i32)> {
    println!("in get_article_articles");
    println!("{}", class);
    let rows = sqlx::query!(
        r#"SELECT view_num, category, favorite_num, comment_num, title, tagname, article_time, articles.id 
        from articles 
        join tags_articles
        on articles.id = tags_articles.article_id
        join tags
        on tags.id = tags_articles.tag_id
        where category = $1
        LIMIT $2
        OFFSET $3"#,
        "222",
        page_size as i64,
        (page_size *(current_page - 1)) as i64
        )
        .fetch_all(pool)
        .await
        .unwrap();
    let mut article = rows.iter()
        .map(|r| (r.view_num, r.category.clone(), r.favorite_num, r.comment_num,
                  r.title.clone(), r.tagname.clone(), r.article_time.to_string(), r.id))
        .collect::<Vec<(i32, String, i32, i32, String, String, String, i32)>>();
    println!("get row.iter() len {}", article.len());
    let mut first_article = article.remove(0); 
    let mut list: Vec<(i32, String, i32, i32, String, Vec<String>, String, i32)> = 
                vec![(first_article.0, first_article.1, first_article.2,
                    first_article.3, first_article.4, vec![first_article.5], first_article.6,
                    first_article.7)];
    for a in article {
       if a.7 == first_article.7 {
            let mut temp = list.pop().unwrap();
            temp.5.push(a.5);
            list.push(temp);
       }else{
           list.push((a.0, a.1.clone(), a.2, a.3, a.4.clone(), vec![a.5.clone()], a.6.clone(), a.7));
           first_article = a;
       }
    };

    list

}*/

/*pub async fn get_article_getByTag(pool: &PgPool, current_page: i32, page_size: i32, tag: String) -> Vec<(i32, String, i32, i32, String, Vec<String>, String, i32)> {
    let key = tag.as_str();
    let rows = sqlx::query!(
        r#"SELECT view_num, category, favorite_num, comment_num, title, tagname, article_time, articles.id 
        from articles 
        join tags_articles
        on articles.id = tags_articles.article_id
        join tags
        on tags.id = tags_articles.tag_id
        where tagname = $1
        LIMIT $2
        OFFSET $3"#,
        "tag2",
        page_size as i64,
        (page_size *(current_page - 1)) as i64
        )
        .fetch_all(pool)
        .await
        .unwrap();

    let mut article = rows.iter()
        .map(|r| (r.view_num, r.category.clone(), r.favorite_num, r.comment_num,
                  r.title.clone(), r.tagname.clone(), r.article_time.to_string(), r.id))
        .collect::<Vec<(i32, String, i32, i32, String, String, String, i32)>>();
    println!("get row.iter()");
    if article.len() == 0 {
        let mut list: Vec<(i32, String, i32, i32, String, Vec<String>, String, i32)> = Vec::new();
        list
    } else {
        let mut first_article = article.remove(0); 
        let mut list: Vec<(i32, String, i32, i32, String, Vec<String>, String, i32)> = 
                    vec![(first_article.0, first_article.1, first_article.2,
                        first_article.3, first_article.4, vec![first_article.5], first_article.6,
                        first_article.7)];
        for a in article {
           if a.7 == first_article.7 {
                let mut temp = list.pop().unwrap();
                temp.5.push(a.5);
                list.push(temp);
           }else{
               list.push((a.0, a.1.clone(), a.2, a.3, a.4.clone(), vec![a.5.clone()], a.6.clone(), a.7));
               first_article = a;
           }
        };

        list
    }
    
}*/

/*pub async fn get_article_articles_keyword1(pool: &PgPool, current_page: i32, page_size: i32, keyword: String) -> Vec<(i32, String, i32, i32, String, Vec<String>, String, i32)> {
    println!("in get_article_articles");
    let key = keyword.as_str();
    let rows = sqlx::query!(
        r#"SELECT view_num, category, favorite_num, comment_num, title, tagname, article_time, articles.id 
        from articles 
        join tags_articles
        on articles.id = tags_articles.article_id
        join tags
        on tags.id = tags_articles.tag_id
        where title LIKE $1
        LIMIT $2
        OFFSET $3"#,
        "mark1",
        page_size as i64,
        (page_size *(current_page - 1)) as i64
        )
        .fetch_all(pool)
        .await
        .unwrap();
    let mut article = rows.iter()
        .map(|r| (r.view_num, r.category.clone(), r.favorite_num, r.comment_num,
                  r.title.clone(), r.tagname.clone(), r.article_time.to_string(), r.id))
        .collect::<Vec<(i32, String, i32, i32, String, String, String, i32)>>();
    println!("get row.iter()");
    let mut first_article = article.remove(0); 
    let mut list: Vec<(i32, String, i32, i32, String, Vec<String>, String, i32)> = 
                vec![(first_article.0, first_article.1, first_article.2,
                    first_article.3, first_article.4, vec![first_article.5], first_article.6,
                    first_article.7)];
    for a in article {
       if a.7 == first_article.7 {
            let mut temp = list.pop().unwrap();
            temp.5.push(a.5);
            list.push(temp);
       }else{
           list.push((a.0, a.1.clone(), a.2, a.3, a.4.clone(), vec![a.5.clone()], a.6.clone(), a.7));
           first_article = a;
       }
    };

    list

}*/

/*pub async fn get_article_articles_keyword2(pool: &PgPool, current_page: i32, page_size: i32, keyword: String) -> Vec<(i32, String, i32, i32, String, Vec<String>, String, i32)> {
    println!("in get_article_articles");
    let rows = sqlx::query!(
        r#"SELECT view_num, category, favorite_num, comment_num, title, tagname, article_time, articles.id 
        from articles 
        join tags_articles
        on articles.id = tags_articles.article_id
        join tags
        on tags.id = tags_articles.tag_id
        where title LIKE $1
        LIMIT $2
        OFFSET $3"#,
        "mark2",
        page_size as i64,
        (page_size *(current_page - 1)) as i64
        )
        .fetch_all(pool)
        .await
        .unwrap();

    let mut article = rows.iter()
        .map(|r| (r.view_num, r.category.clone(), r.favorite_num, r.comment_num,
                  r.title.clone(), r.tagname.clone(), r.article_time.to_string(), r.id))
        .collect::<Vec<(i32, String, i32, i32, String, String, String, i32)>>();
    println!("get row.iter()");
    let mut first_article = article.remove(0); 
    let mut list: Vec<(i32, String, i32, i32, String, Vec<String>, String, i32)> = 
                vec![(first_article.0, first_article.1, first_article.2,
                    first_article.3, first_article.4, vec![first_article.5], first_article.6,
                    first_article.7)];
    for a in article {
       if a.7 == first_article.7 {
            let mut temp = list.pop().unwrap();
            temp.5.push(a.5);
            list.push(temp);
       }else{
           list.push((a.0, a.1.clone(), a.2, a.3, a.4.clone(), vec![a.5.clone()], a.6.clone(), a.7));
           first_article = a;
       }
    };

    list

}*/

/*pub async fn get_article_articles_keyword3(pool: &PgPool, current_page: i32, page_size: i32, keyword: String) -> Vec<(i32, String, i32, i32, String, Vec<String>, String, i32)> {
    println!("in get_article_articles");
    let rows = sqlx::query!(
        r#"SELECT view_num, category, favorite_num, comment_num, title, tagname, article_time, articles.id 
        from articles 
        join tags_articles
        on articles.id = tags_articles.article_id
        join tags
        on tags.id = tags_articles.tag_id
        where title LIKE $1
        LIMIT $2
        OFFSET $3"#,
        "mark3",
        page_size as i64,
        (page_size *(current_page - 1)) as i64
        )
        .fetch_all(pool)
        .await
        .unwrap();

    let mut article = rows.iter()
        .map(|r| (r.view_num, r.category.clone(), r.favorite_num, r.comment_num,
                  r.title.clone(), r.tagname.clone(), r.article_time.to_string(), r.id))
        .collect::<Vec<(i32, String, i32, i32, String, String, String, i32)>>();
    println!("get row.iter()");
    let mut first_article = article.remove(0); 
    let mut list: Vec<(i32, String, i32, i32, String, Vec<String>, String, i32)> = 
                vec![(first_article.0, first_article.1, first_article.2,
                    first_article.3, first_article.4, vec![first_article.5], first_article.6,
                    first_article.7)];
    for a in article {
       if a.7 == first_article.7 {
            let mut temp = list.pop().unwrap();
            temp.5.push(a.5);
            list.push(temp);
       }else{
           list.push((a.0, a.1.clone(), a.2, a.3, a.4.clone(), vec![a.5.clone()], a.6.clone(), a.7));
           first_article = a;
       }
    };

    list

}*/
