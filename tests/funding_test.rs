use payabli_api::prelude::*;

mod wire_test_utils;

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_funding_deposit_funds_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .funding
        .deposit_funds(
            &DepositFundsRequest {
                amount: 10.0,
                entrypoint: Entrypointfield("48acde49".to_string()),
                account_id: "333".to_string(),
                paypoint_id: None,
                same_day_ach: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("POST", "/Funding/depositFunds", None, 1)
        .await
        .unwrap();
}
