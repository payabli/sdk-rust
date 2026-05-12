use payabli_api::prelude::*;

mod wire_test_utils;

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_management_verify_account_details_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    config.environment = None;
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .management
        .verify_account_details(
            &"entry752".to_string(),
            &VerifyAccountDetailsRequest {
                routing_number: "122105278".to_string(),
                account_number: "0000000016".to_string(),
                account_type: Some("Checking".to_string()),
                country: Some("US".to_string()),
                account_holder_type: Some("personal".to_string()),
                holder_name: Some("Jane Doe".to_string()),
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "POST",
        "/Management/verifyAccountDetails/entry752",
        None,
        1,
    )
    .await
    .unwrap();
}
