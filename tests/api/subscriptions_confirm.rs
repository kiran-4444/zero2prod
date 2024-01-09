use crate::helpers::spawn_app;

#[tokio::test]
async fn confirmations_without_tokens_are_rejected_with_a_400() {
    let app = spawn_app().await;

    let resonse = reqwest::get(&format!("{}/subscriptions/confirm", app.address))
        .await
        .unwrap();

    assert_eq!(resonse.status().as_u16(), 400);
}
