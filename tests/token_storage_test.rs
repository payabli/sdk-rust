use payabli_api::prelude::*;

mod wire_test_utils;

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_token_storage_add_method_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .token_storage
        .add_method(
            &AddMethodRequest {
                body: RequestTokenStorage {
                    customer_data: Some(PayorDataRequest {
                        customer_id: Some(CustomerId(4440)),
                        ..Default::default()
                    }),
                    entry_point: Some(Entrypointfield("f743aed24a".to_string())),
                    fallback_auth: Some(true),
                    fallback_auth_amount: Some(100),
                    method_description: Some("Primary Visa card".to_string()),
                    payment_method: Some(RequestTokenStoragePaymentMethod::TokenizeCard(
                        TokenizeCard {
                            method: "card".to_string(),
                            cardcvv: Some(Cardcvv("123".to_string())),
                            cardexp: Cardexp("02/25".to_string()),
                            card_holder: Cardholder("John Doe".to_string()),
                            cardnumber: Cardnumber("4111111111111111".to_string()),
                            cardzip: Some(Cardzip("12345".to_string())),
                            ..Default::default()
                        },
                    )),
                    source: Some(Source("api".to_string())),
                    ..Default::default()
                },
                ach_validation: None,
                create_anonymous: None,
                force_customer_creation: None,
                temporary: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("POST", "/TokenStorage/add", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_token_storage_get_method_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .token_storage
        .get_method(
            &"32-8877drt00045632-678".to_string(),
            &GetMethodQueryRequest {
                card_expiration_format: Some(1),
                include_temporary: Some(false),
                ..Default::default()
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/TokenStorage/32-8877drt00045632-678",
        Some(HashMap::from([
            ("cardExpirationFormat".to_string(), "1".to_string()),
            ("includeTemporary".to_string(), "false".to_string()),
        ])),
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_token_storage_remove_method_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .token_storage
        .remove_method(&"32-8877drt00045632-678".to_string(), None)
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "DELETE",
        "/TokenStorage/32-8877drt00045632-678",
        None,
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_token_storage_update_method_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .token_storage
        .update_method(
            &"32-8877drt00045632-678".to_string(),
            &UpdateMethodRequest {
                body: RequestTokenStorage {
                    customer_data: Some(PayorDataRequest {
                        customer_id: Some(CustomerId(4440)),
                        ..Default::default()
                    }),
                    entry_point: Some(Entrypointfield("f743aed24a".to_string())),
                    fallback_auth: Some(true),
                    payment_method: Some(RequestTokenStoragePaymentMethod::TokenizeCard(
                        TokenizeCard {
                            method: "card".to_string(),
                            cardcvv: Some(Cardcvv("123".to_string())),
                            cardexp: Cardexp("02/25".to_string()),
                            card_holder: Cardholder("John Doe".to_string()),
                            cardnumber: Cardnumber("4111111111111111".to_string()),
                            cardzip: Some(Cardzip("12345".to_string())),
                            ..Default::default()
                        },
                    )),
                    ..Default::default()
                },
                ach_validation: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("PUT", "/TokenStorage/32-8877drt00045632-678", None, 1)
        .await
        .unwrap();
}
