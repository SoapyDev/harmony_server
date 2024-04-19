mod message_create;
mod message_delete;
mod message_update;
mod messages_get;

use actix_web::web;

pub(crate) fn config(config: &mut web::ServiceConfig) {
    config
        .route(
            "/beneficiaries/{id}/messages",
            web::get().to(messages_get::messages_get),
        )
        .route(
            "/beneficiaries/{id}/messages",
            web::post().to(message_create::message_create),
        )
        .route(
            "/beneficiaries/{id}/messages/{message_id}",
            web::put().to(message_update::message_update),
        )
        .route(
            "/beneficiaries/{id}/messages/{message_id}",
            web::delete().to(message_delete::message_delete),
        );
}
