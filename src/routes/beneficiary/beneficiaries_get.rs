use actix_web::{HttpResponse, Responder};

pub(crate) async fn beneficiaries_get() -> impl Responder {
    HttpResponse::Ok()
}
