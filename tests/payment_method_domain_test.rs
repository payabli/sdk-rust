use payabli_api::prelude::*;

mod wire_test_utils;

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_payment_method_domain_add_payment_method_domain_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .payment_method_domain
        .add_payment_method_domain(
            &AddPaymentMethodDomainRequest {
                apple_pay: Some(AddPaymentMethodDomainRequestApplePay {
                    is_enabled: Some(IsEnabled(true)),
                }),
                google_pay: Some(AddPaymentMethodDomainRequestGooglePay {
                    is_enabled: Some(IsEnabled(true)),
                }),
                domain_name: Some(DomainName("checkout.example.com".to_string())),
                entity_id: Some(EntityId(109)),
                entity_type: Some(EntityType("paypoint".to_string())),
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("POST", "/PaymentMethodDomain", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_payment_method_domain_cascade_payment_method_domain_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .payment_method_domain
        .cascade_payment_method_domain(&"pmd_b8237fa45c964d8a9ef27160cd42b8c5".to_string(), None)
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "POST",
        "/PaymentMethodDomain/pmd_b8237fa45c964d8a9ef27160cd42b8c5/cascade",
        None,
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_payment_method_domain_delete_payment_method_domain_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .payment_method_domain
        .delete_payment_method_domain(&"pmd_b8237fa45c964d8a9ef27160cd42b8c5".to_string(), None)
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "DELETE",
        "/PaymentMethodDomain/pmd_b8237fa45c964d8a9ef27160cd42b8c5",
        None,
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_payment_method_domain_get_payment_method_domain_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .payment_method_domain
        .get_payment_method_domain(&"pmd_b8237fa45c964d8a9ef27160cd42b8c5".to_string(), None)
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/PaymentMethodDomain/pmd_b8237fa45c964d8a9ef27160cd42b8c5",
        None,
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_payment_method_domain_list_payment_method_domains_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .payment_method_domain
        .list_payment_method_domains(
            &ListPaymentMethodDomainsQueryRequest {
                entity_id: Some(1147),
                entity_type: Some("paypoint".to_string()),
                from_record: None,
                limit_record: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/PaymentMethodDomain/list",
        Some(HashMap::from([
            ("entityId".to_string(), "1147".to_string()),
            ("entityType".to_string(), "paypoint".to_string()),
        ])),
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_payment_method_domain_update_payment_method_domain_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .payment_method_domain
        .update_payment_method_domain(
            &"pmd_b8237fa45c964d8a9ef27160cd42b8c5".to_string(),
            &UpdatePaymentMethodDomainRequest {
                apple_pay: Some(UpdatePaymentMethodDomainRequestWallet {
                    is_enabled: Some(IsEnabled(false)),
                }),
                google_pay: Some(UpdatePaymentMethodDomainRequestWallet {
                    is_enabled: Some(IsEnabled(false)),
                }),
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "PATCH",
        "/PaymentMethodDomain/pmd_b8237fa45c964d8a9ef27160cd42b8c5",
        None,
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_payment_method_domain_verify_payment_method_domain_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .payment_method_domain
        .verify_payment_method_domain(&"pmd_b8237fa45c964d8a9ef27160cd42b8c5".to_string(), None)
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "POST",
        "/PaymentMethodDomain/pmd_b8237fa45c964d8a9ef27160cd42b8c5/verify",
        None,
        1,
    )
    .await
    .unwrap();
}
