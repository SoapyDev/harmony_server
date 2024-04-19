use actix_web::{HttpResponse, Responder};

pub(crate) async fn user_get() -> impl Responder {
    HttpResponse::Ok()
}
