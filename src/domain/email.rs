use validator::validate_email;

#[derive(Debug)]
pub struct Email(String);

impl Email {
    pub fn parse(s: String) -> Result<Self, &'static str> {
        if validate_email(&s) {
            Ok(Self(s))
        } else {
            Err("Courriel invalide")
        }
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
