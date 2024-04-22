use claims::{assert_err, assert_ok};
use fake::faker::internet::en::SafeEmail;
use fake::Fake;
use harmony_server::domain::email::Email;

#[test]
fn email_is_valid() {
    for _ in 0..10 {
        let email = SafeEmail().fake();
        assert_ok!(Email::parse(email));
    }
}

#[test]
fn email_without_at_symbol() {
    let email: String = SafeEmail().fake();
    assert_err!(Email::parse(email.replace("@", "")));
}

#[test]
fn email_without_domain() {
    let email = "test@test";
    assert_ok!(Email::parse(email.to_string()));
}

#[test]
fn email_without_username() {
    let email = "@test.com";
    assert_err!(Email::parse(email.to_string()));
}

#[test]
fn email_is_empty() {
    let email = "";
    assert_err!(Email::parse(email.to_string()));
}

#[test]
fn email_is_whitespace() {
    let email = "            ";
    assert_err!(Email::parse(email.to_string()));
}

#[test]
fn email_has_multiple_domain() {
    let email = "test@test2.test.com";
    assert_ok!(Email::parse(email.to_string()));
}

#[test]
fn email_is_short() {
    let email = "";
    assert_err!(Email::parse(email.to_string()));
}

#[test]
fn email_is_long() {
    let email = "a".repeat(142);
    let email = email + "@test.com";
    assert_err!(Email::parse(email.to_string()));
}
