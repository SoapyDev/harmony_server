use actix_web::{HttpResponse, Responder};
pub(crate) async fn message_create() -> impl Responder {
    HttpResponse::Ok()
}
