use actix_web::{HttpResponse, Responder};

pub(crate) async fn message_update() -> impl Responder {
    HttpResponse::Ok()
}
