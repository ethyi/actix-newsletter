use crate::helpers::spawn_app;
#[tokio::test]
async fn health_check_works() {
    // setup
    let test_app = spawn_app().await;
    // get http requests against our application
    let client = reqwest::Client::new();
    // Act
    let response = client
        .get(&format!("{}/health_check", &test_app.address))
        .send()
        .await
        .expect("Failed to execute request.");
    // asserts
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
