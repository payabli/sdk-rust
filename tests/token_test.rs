use payabli_api::prelude::*;

mod wire_test_utils;

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_token_create_server_side_token_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .token
        .create_server_side_token(
            &CreateServerSideTokenRequest {
                client_id: "YOUR_CLIENT_ID".to_string(),
                client_secret: "YOUR_CLIENT_SECRET".to_string(),
                state: None,
                permissions: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("POST", "/v2/Token/serverside", None, 1)
        .await
        .unwrap();
}
