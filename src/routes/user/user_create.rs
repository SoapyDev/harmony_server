use actix_web::{HttpResponse, Responder};

pub(crate) async fn user_create() -> impl Responder {
    HttpResponse::Ok()
}
