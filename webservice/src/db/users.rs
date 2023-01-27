use crate::{models::user::*, state::response_state, errors::MyError};
use sqlx::{postgres::PgPool, Executor};

pub async fn post_user_login(pool: &PgPool, email: String, password: String) 
    -> Result<response_state::LoginResponse, MyError>{
/*    let user = sqlx::query!(
        r#"SELECT id, account_type, avator
        FROM users
        WHERE email = $1
        AND password = $2"#,
        email,
        password
        )
        .fetch_one(pool)
        .await?;*/

    Ok(response_state::LoginResponse{
        nick_name: "nick_name".to_string(),
        user_id: 1,
        email: "email".to_string(),
        avatar: "avatar".to_string(),
        token: "token".to_string(),
        account_type: "account_type".to_string()
    }) 
}

pub async fn get_user_account(pool: &PgPool, email: String, password: String) 
    -> Result<response_state::UserInfoResponse, MyError> {
/*    let user = sqlx::query!(
        r#"SELECT id, account_type, avator
        FROM users
        WHERE email = $1
        AND password = $2"#,
        email,
        password
        )
        .fetch_one(pool)
        .await?;*/

    Ok(response_state::UserInfoResponse{
        nick_name: "nick_name".to_string(),
        user_id: 1,
        email: "email".to_string(),
        avatar: "avatar".to_string(),
        article_num: 10,
        read_num: 2,
        like_num: 1
    }) 


}

/*pub async fn post_user_register(pool: &PgPool, name: String, email: String, password: String) -> bool {
    println!("now in postegres");
    let row = sqlx::query!(

            r#"INSERT INTO users (username, email, password, avatar, account_type)
            VALUES ($1, $2, $3, $4, $5)"#,
            name,
            email,
            password,
            "123".to_string(),
            true
        )
        .fetch_one(pool)
        .await;

   match row {
       PgRow => true,
       Error => false
   }
}*/

/*pub async fn get_user_register(pool: &PgPool, email: String, password: String) -> (i32, String, String, i32) {
    let user = sqlx::query!(
        r#"SELECT id, email, account_type, avatar
        FROM users
        WHERE email = $1
        AND password = $2"#,
        email,
        password
        )
        .fetch_one(pool)
        .await
        .unwrap();

    (user.id, user.email.clone(), user.avatar.clone(), 1)
}*/

/*pub async fn get_user_register(pool: &PgPool, email: String, password: String) -> (i32, String, String, i32) {
    let user = sqlx::query!(
        r#"SELECT id, email, account_type, avatar
        FROM users
        WHERE email = $1
        AND password = $2"#,
        email,
        password
        )
        .fetch_one(pool)
        .await
        .unwrap();

    (user.id, user.email.clone(), user.avatar.clone(), 1)
}*/
