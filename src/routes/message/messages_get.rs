use actix_web::{HttpResponse, Responder};
pub(crate) async fn messages_get() -> impl Responder {
    HttpResponse::Ok()
}
