mod transaction_create;
mod transaction_delete;
mod transaction_update;
mod transactions_get;

use actix_web::web;

pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.route(
        "/beneficiaries/{id}/transactions",
        web::get().to(transactions_get::transactions_get),
    )
    .route(
        "/beneficiaries/{id}/transactions",
        web::post().to(transaction_create::transaction_create),
    )
    .route(
        "/beneficiaries/{id}/transactions/{transaction_id}",
        web::put().to(transaction_update::transaction_update),
    )
    .route(
        "/beneficiaries/{id}/transactions/{transaction_id}",
        web::delete().to(transaction_delete::transaction_delete),
    );
}
