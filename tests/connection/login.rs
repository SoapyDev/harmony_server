use crate::spawn_app;

#[tokio::test]
async fn login_works() {
    let address = spawn_app().await;
    let client = reqwest::Client::new();

    let body = serde_json::json!({
        "username": "SoapyDev",
        "password": "Abea357!",
    });

    let response = client
        .post(&format!("{}/login", &address))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
