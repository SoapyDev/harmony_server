use secrecy::{ExposeSecret, SecretString};
use uuid::Uuid;

#[derive(Debug, serde::Deserialize)]
pub struct User {
    pub(crate) id: Uuid,
    pub(crate) first_name: SecretString,
    pub(crate) last_name: SecretString,
    #[serde(serialize_with = "serialize_secret_string")]
    pub(crate) password: SecretString,
    pub(crate) email: SecretString,
    pub(crate) phone: SecretString,
    pub(crate) role: String,
    pub(crate) birth_date: chrono::NaiveDate,
    pub(crate) starting_date: chrono::NaiveDate,
    pub(crate) created_at: chrono::NaiveDateTime,
}

#[allow(unused)]
pub(crate) fn serialize_secret_string<S>(
    secret: &SecretString,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&secret.expose_secret())
}

#[derive(serde::Serialize)]
pub(crate) struct ResponseUser {
    pub(crate) id: Uuid,
    pub(crate) first_name: String,
    pub(crate) last_name: String,
    pub(crate) password: String,
    pub(crate) email: String,
    pub(crate) phone: String,
    pub(crate) role: String,
    pub(crate) birth_date: chrono::NaiveDate,
    pub(crate) starting_date: chrono::NaiveDate,
    pub(crate) created_at: chrono::NaiveDateTime,
}

impl From<User> for ResponseUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            first_name: user.first_name.expose_secret().to_string(),
            last_name: user.last_name.expose_secret().to_string(),
            password: "".to_string(),
            email: user.email.expose_secret().to_string(),
            phone: user.phone.expose_secret().to_string(),
            role: user.role,
            birth_date: user.birth_date,
            starting_date: user.starting_date,
            created_at: user.created_at,
        }
    }
}
