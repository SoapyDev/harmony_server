mod disconnect;
mod health_check;
mod login;

use actix_web::web;

pub(crate) fn config(config: &mut web::ServiceConfig) {
    config
        .route("/health_check", web::get().to(health_check::health_check))
        .route("/disconnect", web::post().to(disconnect::disconnect))
        .route("/login", web::post().to(login::login));
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct User {
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

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Connections {
    pub id: String,
    pub username: String,
    pub created_at: chrono::NaiveDateTime,
    pub expires_at: chrono::NaiveDateTime,
}
