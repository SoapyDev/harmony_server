use actix_web::{HttpResponse, Responder};
pub(crate) async fn users_get() -> impl Responder {
    HttpResponse::Ok()
}
