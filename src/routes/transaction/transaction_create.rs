use actix_web::{HttpResponse, Responder};

pub(crate) async fn transaction_create() -> impl Responder {
    HttpResponse::Ok()
}
