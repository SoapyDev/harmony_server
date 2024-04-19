use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

pub(crate) async fn login(data: web::Json<UserLogin>, pool: web::Data<PgPool>) -> impl Responder {
    let user = match sqlx::query_as!(
        User,
        r#"SELECT * FROM users WHERE username = $1 AND password = $2"#,
        data.username,
        data.password
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(user) => Ok(user),
        Err(_) => Err("Invalid username or password"),
    };

    if let Ok(user) = user {
        println!("{:?}", user);
    }

    HttpResponse::Ok()
}

#[derive(serde::Deserialize)]
pub(crate) struct UserLogin {
    username: String,
    password: String,
}

#[derive(Debug)]
pub(crate) struct User {
    id: i32,
    first_name: String,
    last_name: String,
    username: String,
    password: String,
    email: String,
    phone: String,
    role: String,
    birth_date: chrono::NaiveDate,
    starting_date: chrono::NaiveDate,
    created_at: chrono::NaiveDateTime,
}
