use actix_web::{HttpResponse, Responder};
pub(crate) async fn transactions_get() -> impl Responder {
    HttpResponse::Ok()
}
