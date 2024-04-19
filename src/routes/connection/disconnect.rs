use actix_web::{HttpResponse, Responder};

pub(crate) async fn disconnect() -> impl Responder {
    HttpResponse::Ok()
}
