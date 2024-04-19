use crate::spawn_app;

#[tokio::test]
async fn allergy_delete_works() {
    let address = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .delete(&format!("{}/beneficiaries/1/allergies/1", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
