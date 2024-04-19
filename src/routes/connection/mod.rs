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
