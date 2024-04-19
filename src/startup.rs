use crate::routes::{allergy, beneficiary, connection, message, transaction, user};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .configure(connection::config)
            .configure(beneficiary::config)
            .configure(allergy::config)
            .configure(message::config)
            .configure(transaction::config)
            .configure(user::config)
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
