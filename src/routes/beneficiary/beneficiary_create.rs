use actix_web::{HttpResponse, Responder};

pub(crate) async fn beneficiary_create() -> impl Responder {
    HttpResponse::Ok()
}
