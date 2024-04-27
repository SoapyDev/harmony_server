use std::net::TcpListener;

use actix_cors::Cors;
use actix_session::storage::RedisSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::dev::Server;
use actix_web::{middleware::Compress, web, App, HttpServer};
use actix_web_lab::middleware::from_fn;
use secrecy::{ExposeSecret, Secret};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;

use crate::authentication::reject_anonymous_users;
use crate::configurations::{DatabaseSettings, Settings};
use crate::routes::{allergy, beneficiary, health_check, login, message, transaction, user};

pub struct Application {
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        let connection_pool = get_connection_pool(&configuration.database)
            .await
            .expect("Failed to connect to postgres");

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );

        let listener = TcpListener::bind(address)?;

        let server = run(
            listener,
            connection_pool,
            configuration.application.hmac_secret,
            configuration.redis_uri,
            configuration.web_uri,
        )
        .await?;

        Ok(Self { server })
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub async fn get_connection_pool(configuration: &DatabaseSettings) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .connect_with(configuration.with_db())
        .await
}

pub async fn run(
    listener: TcpListener,
    db_pool: PgPool,
    hmac_secret: Secret<String>,
    redis_uri: Secret<String>,
    web_uri: Secret<String>,
) -> Result<Server, anyhow::Error> {
    let connection = web::Data::new(db_pool);
    let secret_key = Key::from(hmac_secret.expose_secret().as_bytes());
    let redis_store = RedisSessionStore::new(redis_uri.expose_secret()).await?;
    let server = HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_header()
                    .allowed_origin(&format!("{}/login",web_uri.expose_secret()))
                    .allowed_methods(vec!["GET", "POST"])
                    .block_on_origin_mismatch(true),
            )
            .wrap(Compress::default())
            .wrap(SessionMiddleware::new(
                redis_store.clone(),
                secret_key.clone(),
            ))
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check::health_check))
            .route("/login", web::post().to(login::login))
            .service(
                web::scope("/app")
                    .wrap(Cors::default().allow_any_origin().allow_any_method().allow_any_header())
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
