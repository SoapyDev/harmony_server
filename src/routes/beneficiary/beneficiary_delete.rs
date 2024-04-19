use actix_web::{HttpResponse, Responder};

pub(crate) async fn beneficiary_delete() -> impl Responder {
    HttpResponse::Ok()
}
