use actix_web::{HttpResponse, Responder};
pub(crate) async fn transaction_delete() -> impl Responder {
    HttpResponse::Ok()
}
