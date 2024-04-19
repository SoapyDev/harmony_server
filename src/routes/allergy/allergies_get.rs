use actix_web::{HttpResponse, Responder};

pub(crate) async fn allergies_get() -> impl Responder {
    HttpResponse::Ok()
}
