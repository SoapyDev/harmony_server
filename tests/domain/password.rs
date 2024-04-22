use claims::{assert_err, assert_ok};
use harmony_server::domain::password::Password;

#[test]
fn password_is_short() {
    let password = "abea357";

    assert_err!(Password::parse(password.to_string()));
}

#[test]
fn password_is_long() {
    let password = "A!3a".repeat(15);
    assert_err!(Password::parse(password.to_string()));
}

#[test]
fn password_has_no_numbers() {
    let password = "Abeaabea!";
    assert_err!(Password::parse(password.to_string()));
}

#[test]
fn password_has_no_symbols() {
    let password = "Abea3571";
    assert_err!(Password::parse(password.to_string()));
}

#[test]
fn password_has_no_uppercase() {
    let password = "abea357!";
    assert_err!(Password::parse(password.to_string()));
}

#[test]
fn password_has_no_lowercase() {
    let password = "ABEA357!";
    assert_err!(Password::parse(password.to_string()));
}

#[test]
fn password_is_valid() {
    let password = "Abea357!";
    assert_ok!(Password::parse(password.to_string()));
}
