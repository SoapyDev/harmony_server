use actix_web::{HttpResponse, Responder};
pub(crate) async fn transaction_update() -> impl Responder {
    HttpResponse::Ok()
}
