use payabli_api::prelude::*;

mod wire_test_utils;

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_hosted_payment_pages_load_page_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .hosted_payment_pages
        .load_page(
            &"8cfec329267".to_string(),
            &"pay-your-fees-1".to_string(),
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/Paypoint/load/8cfec329267/pay-your-fees-1",
        None,
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_hosted_payment_pages_new_page_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .hosted_payment_pages
        .new_page(
            &"8cfec329267".to_string(),
            &PayabliPages {
                additional_data: None,
                credentials: None,
                last_access: None,
                page_content: None,
                page_identifier: None,
                page_settings: None,
                published: None,
                receipt_content: None,
                subdomain: None,
                total_amount: None,
                validation_code: None,
            },
            Some(
                RequestOptions::new()
                    .additional_header("idempotencyKey", "6B29FC40-CA47-1067-B31D-00DD010662DA"),
            ),
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("POST", "/Paypoint/8cfec329267", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_hosted_payment_pages_save_page_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .hosted_payment_pages
        .save_page(
            &"8cfec329267".to_string(),
            &"pay-your-fees-1".to_string(),
            &PayabliPages {
                additional_data: None,
                credentials: None,
                last_access: None,
                page_content: None,
                page_identifier: None,
                page_settings: None,
                published: None,
                receipt_content: None,
                subdomain: None,
                total_amount: None,
                validation_code: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("PUT", "/Paypoint/8cfec329267/pay-your-fees-1", None, 1)
        .await
        .unwrap();
}
