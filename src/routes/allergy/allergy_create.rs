use actix_web::{HttpResponse, Responder};

pub(crate) async fn allergy_create() -> impl Responder {
    HttpResponse::Ok()
}
