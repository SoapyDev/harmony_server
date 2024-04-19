use actix_web::{HttpResponse, Responder};
pub(crate) async fn user_update() -> impl Responder {
    HttpResponse::Ok()
}
