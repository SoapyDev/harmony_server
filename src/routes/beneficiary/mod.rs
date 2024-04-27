pub(crate) mod beneficiaries_get;
pub(crate) mod beneficiary_create;
pub(crate) mod beneficiary_delete;
pub(crate) mod beneficiary_get;
pub(crate) mod beneficiary_update;

use actix_web::web;

pub(crate) fn config(config: &mut web::ServiceConfig) {
    config
        .route(
            "/beneficiaries",
            web::get().to(beneficiaries_get::beneficiaries_get),
        )
        .route(
            "/beneficiaries",
            web::post().to(beneficiary_create::beneficiary_create),
        )
        .route(
            "/beneficiaries/{id}",
            web::get().to(beneficiary_get::beneficiary_get),
        )
        .route(
            "/beneficiaries/{id}",
            web::put().to(beneficiary_update::beneficiary_update),
        )
        .route(
            "/beneficiaries/{id}",
            web::delete().to(beneficiary_delete::beneficiary_delete),
        );
}
