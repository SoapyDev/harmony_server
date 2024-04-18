use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/login", web::post().to(login))
            .route("/beneficiaries", web::get().to(get_beneficiaries))
            .route("/beneficiaries/{id}", web::get().to(get_beneficiary))
            .route("/beneficiaries", web::post().to(create_beneficiary))
            .route("/beneficiaries/{id}", web::put().to(update_beneficiary))
            .route("/beneficiaries/{id}", web::delete().to(delete_beneficiary))
            .route("/beneficiaries/{id}/messages", web::get().to(get_messages))
            .route(
                "/beneficiaries/{id}/messages",
                web::post().to(create_message),
            )
            .route(
                "/beneficiaries/{id}/messages/{message_id}",
                web::put().to(update_message),
            )
            .route(
                "/beneficiaries/{id}/messages/{message_id}",
                web::delete().to(delete_message),
            )
            .route(
                "/beneficiaries/{id}/transactions",
                web::get().to(get_transactions),
            )
            .route(
                "/beneficiaries/{id}/transactions",
                web::post().to(create_transaction),
            )
            .route(
                "/beneficiaries/{id}/transactions/{transaction_id}",
                web::put().to(update_transaction),
            )
            .route(
                "/beneficiaries/{id}/transactions/{transaction_id}",
                web::delete().to(delete_transaction),
            )
            .route(
                "/beneficiaries/{id}/allergies",
                web::get().to(get_allergies),
            )
            .route(
                "/beneficiaries/{id}/allergies",
                web::post().to(create_allergy),
            )
            .route(
                "/beneficiaries/{id}/allergies/{allergy_id}",
                web::delete().to(delete_allergy),
            )
            .route("/users", web::get().to(get_users))
            .route("/users", web::post().to(create_user))
            .route("/users/{id}", web::get().to(get_user))
            .route("/users/{id}", web::put().to(update_user))
            .route("/users/{id}", web::delete().to(delete_user))
    })
    .listen(listener)?
    .run();

    Ok(server)
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

async fn login() -> impl Responder {
    HttpResponse::Ok()
}

async fn get_beneficiaries() -> impl Responder {
    HttpResponse::Ok()
}

async fn get_beneficiary() -> impl Responder {
    HttpResponse::Ok()
}

async fn create_beneficiary() -> impl Responder {
    HttpResponse::Ok()
}

async fn update_beneficiary() -> impl Responder {
    HttpResponse::Ok()
}

async fn delete_beneficiary() -> impl Responder {
    HttpResponse::Ok()
}

async fn get_messages() -> impl Responder {
    HttpResponse::Ok()
}

async fn create_message() -> impl Responder {
    HttpResponse::Ok()
}

async fn update_message() -> impl Responder {
    HttpResponse::Ok()
}

async fn delete_message() -> impl Responder {
    HttpResponse::Ok()
}

async fn get_transactions() -> impl Responder {
    HttpResponse::Ok()
}

async fn create_transaction() -> impl Responder {
    HttpResponse::Ok()
}

async fn update_transaction() -> impl Responder {
    HttpResponse::Ok()
}

async fn delete_transaction() -> impl Responder {
    HttpResponse::Ok()
}

async fn get_allergies() -> impl Responder {
    HttpResponse::Ok()
}

async fn create_allergy() -> impl Responder {
    HttpResponse::Ok()
}

async fn delete_allergy() -> impl Responder {
    HttpResponse::Ok()
}

async fn get_users() -> impl Responder {
    HttpResponse::Ok()
}

async fn create_user() -> impl Responder {
    HttpResponse::Ok()
}

async fn get_user() -> impl Responder {
    HttpResponse::Ok()
}

async fn update_user() -> impl Responder {
    HttpResponse::Ok()
}

async fn delete_user() -> impl Responder {
    HttpResponse::Ok()
}
