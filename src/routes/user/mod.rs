mod user_create;
mod user_delete;
mod user_get;
mod user_update;
mod users_get;

use actix_web::web;
pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/users", web::get().to(users_get::users_get))
        .route("/users", web::post().to(user_create::user_create))
        .route("/users/{id}", web::get().to(user_get::user_get))
        .route("/users/{id}", web::put().to(user_update::user_update))
        .route("/users/{id}", web::delete().to(user_delete::user_delete));
}
