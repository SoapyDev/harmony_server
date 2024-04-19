use actix_web::{HttpResponse, Responder};

pub(crate) async fn allergy_delete() -> impl Responder {
    HttpResponse::Ok()
}
