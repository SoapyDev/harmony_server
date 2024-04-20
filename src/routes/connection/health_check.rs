use actix_web::{HttpResponse, Responder};

#[tracing::instrument(skip(), name = "Health check")]
pub(crate) async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}
