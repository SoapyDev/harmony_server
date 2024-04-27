use secrecy::Zeroize;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct Password(String);

impl Password {
    pub fn parse(s: String) -> Result<Self, &'static str> {
        let s = s.trim();

        let is_short = s.graphemes(true).count() < 8;
        let is_long = s.graphemes(true).count() > 50;
        let has_numbers = s.chars().any(|c| c.is_ascii_digit());
        let is_alphabetic = s.chars().any(|c| !c.is_ascii_alphabetic());
        let has_uppercase = s.chars().any(|c| c.is_ascii_uppercase());
        let has_lowercase = s.chars().any(|c| c.is_ascii_lowercase());

        let special_chars = ['!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '+'];
        let has_special = s.chars().any(|c| special_chars.contains(&c));

        if is_short
            || is_long
            || !has_numbers
            || !is_alphabetic
            || !has_uppercase
            || !has_lowercase
            || !has_special
        {
            return Err("Un mot de passe doit contenir au moins une majuscule, une minuscule, un chiffre et un caractère speciaux et contenir au moins 8 caractères");
        }

        Ok(Self(s.to_string()))
    }
}

impl Zeroize for Password {
    fn zeroize(&mut self) {
        self.0.zeroize();
    }
}
impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
