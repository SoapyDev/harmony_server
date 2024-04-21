mod disconnect;
mod health_check;
mod login;

use actix_web::web;
use secrecy::{ExposeSecret, SecretString};

pub(crate) fn config(config: &mut web::ServiceConfig) {
    config
        .route("/health_check", web::get().to(health_check::health_check))
        .route("/disconnect", web::post().to(disconnect::disconnect))
        .route("/login", web::post().to(login::login));
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    id: i32,
    first_name: SecretString,
    last_name: SecretString,
    username: String,
    #[serde(serialize_with = "serialize_secret_string")]
    password: SecretString,
    email: SecretString,
    phone: SecretString,
    role: String,
    birth_date: chrono::NaiveDate,
    starting_date: chrono::NaiveDate,
    created_at: chrono::NaiveDateTime,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Connections {
    pub id: String,
    pub username: String,
    pub created_at: chrono::NaiveDateTime,
    pub expires_at: chrono::NaiveDateTime,
}

#[allow(unused)]
pub(crate) fn serialize_secret_string<S>(
    secret: &SecretString,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&secret.expose_secret())
}
