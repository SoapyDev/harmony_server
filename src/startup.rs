use crate::authentication::reject_anonymous_users;
use crate::routes::{allergy, beneficiary, health_check, login, message, transaction, user};
use actix_web::dev::Server;
use actix_web::{middleware::Compress, web, App, HttpServer};
use actix_web_lab::middleware::from_fn;
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Compress::default())
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check::health_check))
            .route("/login", web::post().to(login::login))
            .service(
                web::scope("/app")
                    .wrap(from_fn(reject_anonymous_users))
                    .configure(beneficiary::config)
                    .configure(allergy::config)
                    .configure(message::config)
                    .configure(transaction::config)
                    .configure(user::config),
            )
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
