use crate::authentication::AuthError;
use crate::authentication::{validate_credentials, Credentials};
use crate::domain::user::ResponseUser;
use crate::routes::error_chain_fmt;
use crate::session_state::TypedSession;
use actix_web::error::InternalError;
use actix_web::http::header::ContentType;
use actix_web::web;
use actix_web::HttpResponse;
use sqlx::PgPool;

#[tracing::instrument(
    skip(data, pool,session),
    fields(email=data.email, user_id=tracing::field::Empty)
)]
pub(crate) async fn login(
    data: web::Json<Credentials>,
    pool: web::Data<PgPool>,
    session: TypedSession,
) -> Result<HttpResponse, InternalError<LoginError>> {
    let credentials = data.into_inner();
    match validate_credentials(credentials, &pool).await {
        Ok(user) => {
            let _ = session
                .insert_user_id(user.id)
                .map_err(|e| LoginError::UnexpectedError(e.into()));

            let body = match serde_json::to_string(&ResponseUser::from(user)) {
                Ok(body) => body,
                Err(e) => {
                    return Err(LoginError::UnexpectedError(e.into()).into());
                }
            };

            Ok(HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(body))
        }
        Err(e) => match e {
            AuthError::InvalidCredentials(_) => Ok(HttpResponse::Unauthorized().finish()),
            AuthError::UnexpectedError(_) => Err(LoginError::UnexpectedError(e.into()).into()),
        },
    }
}

#[derive(thiserror::Error)]
pub enum LoginError {
    #[error("Authentication failed")]
    AuthError(#[source] anyhow::Error),
    #[error("Something went wrong")]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl Into<InternalError<LoginError>> for LoginError {
    fn into(self) -> InternalError<LoginError> {
        InternalError::from_response(self, HttpResponse::InternalServerError().finish())
    }
}
