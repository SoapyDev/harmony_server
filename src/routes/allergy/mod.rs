mod allergies_get;
mod allergy_create;
mod allergy_delete;

use actix_web::web;

pub(crate) fn config(config: &mut web::ServiceConfig) {
    config
        .route(
            "/beneficiaries/{id}/allergies",
            web::get().to(allergies_get::allergies_get),
        )
        .route(
            "/beneficiaries/{id}/allergies",
            web::post().to(allergy_create::allergy_create),
        )
        .route(
            "/beneficiaries/{id}/allergies/{allergy_id}",
            web::delete().to(allergy_delete::allergy_delete),
        );
}
