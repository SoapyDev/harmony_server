use actix_web::{HttpResponse, Responder};

pub(crate) async fn message_delete() -> impl Responder {
    HttpResponse::Ok()
}
