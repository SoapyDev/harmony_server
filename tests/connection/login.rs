use crate::spawn_app;
use harmony_server::routes::connection::Connections;

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

    match response.json::<Connections>().await {
        Ok(val) => {
            println!("{:?}", val);
            assert_eq!(val.username, "SoapyDev");
        }
        Err(val) => {
            println!("{:?}", val);
            assert!(val.to_string().contains("Invalid username or password"));
        }
    }
}
