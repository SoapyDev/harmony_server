use actix_web::{HttpResponse, Responder};
pub(crate) async fn beneficiary_update() -> impl Responder {
    HttpResponse::Ok()
}
