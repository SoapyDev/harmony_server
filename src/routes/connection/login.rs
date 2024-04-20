use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use tracing::Instrument;

use crate::routes::connection::{Connections, User};

use uuid::{Timestamp, Uuid};
use secrecy::{Secret, ExposeSecret};


#[tracing::instrument(name="Login user", 
    skip(data, pool),
    fields(
        username = %data.username, 
    ))]

pub(crate) async fn login(data: web::Json<UserLogin>, pool: web::Data<PgPool>) -> impl Responder {

    let result = match get_user_if_exists(&data, &pool).await{
        Ok(user) => user,
        Err(_) => {
            return HttpResponse::Unauthorized().body("Invalid username or password");
        }
    };

    match get_connection_if_exists(&result, &pool)
        .await
    {
        Some(connection) => {
            return HttpResponse::Ok().json(connection);
        }
        None => {}
    }

    match create_connection(&result, &pool)
        .await
    {
        Ok(_) => {}
        Err(e) => {
            return HttpResponse::InternalServerError().body("Failed to create connection: ".to_owned() + e);
        }
    };

    match get_connection_if_exists(&result, &pool)
        .await
    {
        Some(connection) => {
            HttpResponse::Ok().json(connection)
        }
        None => {
            return HttpResponse::InternalServerError().body("Failed to retrieve connection".to_owned());
        }
    }
}


#[tracing::instrument(
    name="Create connection", 
    skip(result, pool),
    fields(
        username = %result.username,
))]

async fn create_connection(result: &User, pool: &web::Data<PgPool>) -> Result<(), &'static str> {
    let timestamp = Timestamp::now(uuid::NoContext);
    let uuid = Uuid::new_v7(timestamp);

    match sqlx::query!(
        r#"INSERT INTO connections (id,username) VALUES ($1,$2)"#,
        uuid,
        result.username,
    )
    .execute(pool.get_ref())
    .instrument(tracing::Span::current())
    .await
    {
        Ok(_) => {
            tracing::info!("Connection created");
            Ok(())
        },
        Err(e) => {
            tracing::error!("{}",format!("Failed to create connection : {}", e.to_string()));
            Err("Failed to create connection")
        },
    }
}





#[tracing::instrument(
    name="Get user from database if exists", 
    skip(data, pool),
    fields(
        username = %data.username,
))]

async fn get_user_if_exists(
    data: &web::Json<UserLogin>,
    pool: &web::Data<PgPool>,
) -> Result<User, &'static str> {
    match sqlx::query_as!(
        User,
        r#"SELECT * FROM users WHERE username = $1 AND password = $2"#,
        data.username,
        data.password.expose_secret()
    )
    .fetch_one(pool.get_ref())
    .instrument(tracing::Span::current())
    .await
    {
        Ok(user) => {
            tracing::info!("User found : {:?}", user);
            Ok(user)
        },
        Err(e) => {
            tracing::error!("Invalid username or password : {}", e.to_string());
            Err("Invalid username or password")
        },
    }
}



#[tracing::instrument(
    name="Get connection from database if exists", 
    skip(result, pool),
    fields(
        username = %result.username,
))]

async fn get_connection_if_exists(result: &User, pool: &web::Data<PgPool>) -> Option<Connections> {
    match sqlx::query_as!(
        Connections,
        r#"SELECT * FROM connections WHERE username = $1 AND expires_at > NOW()"#,
        &result.username,
    )
    .fetch_optional(pool.get_ref())
    .await
    {
        Ok(Some(connection)) => {
            tracing::info!("Connection found : {:?}", connection);
            Some(connection)
        }
        _ => {
            tracing::info!("Connection not found");
            None
        }
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct UserLogin {
    username: String,
    password: Secret<String>,
}
