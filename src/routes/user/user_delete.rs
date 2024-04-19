use actix_web::{HttpResponse, Responder};

pub(crate) async fn user_delete() -> impl Responder {
    HttpResponse::Ok()
}
