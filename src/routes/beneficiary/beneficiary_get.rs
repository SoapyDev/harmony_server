use actix_web::{HttpResponse, Responder};
pub(crate) async fn beneficiary_get() -> impl Responder {
    HttpResponse::Ok()
}
