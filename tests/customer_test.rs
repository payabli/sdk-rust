use payabli_api::prelude::*;

mod wire_test_utils;

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_customer_add_customer_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .customer
        .add_customer(
            &Entrypointfield("8cfec329267".to_string()),
            &AddCustomerRequest {
                body: CustomerData {
                    customer_number: Some(CustomerNumberNullable("12356ACB".to_string())),
                    customer_username: None,
                    customer_psw: None,
                    customer_status: None,
                    company: None,
                    firstname: Some("Irene".to_string()),
                    lastname: Some("Canizales".to_string()),
                    phone: None,
                    email: Some(Email("irene@canizalesconcrete.com".to_string())),
                    address: None,
                    address_1: Some("123 Bishop's Trail".to_string()),
                    city: Some("Mountain City".to_string()),
                    state: Some("TN".to_string()),
                    zip: Some("37612".to_string()),
                    country: Some("US".to_string()),
                    shipping_address: None,
                    shipping_address_1: None,
                    shipping_city: None,
                    shipping_state: None,
                    shipping_zip: None,
                    shipping_country: None,
                    balance: None,
                    time_zone: Some(Timezone(-5)),
                    additional_fields: None,
                    identifier_fields: Some(Identifierfields(vec![Some("email".to_string())])),
                    created_at: None,
                },
                force_customer_creation: None,
                replace_existing: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("POST", "/Customer/single/8cfec329267", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_customer_delete_customer_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client.customer.delete_customer(998, None).await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("DELETE", "/Customer/998", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_customer_get_customer_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client.customer.get_customer(998, None).await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("GET", "/Customer/998", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_customer_link_customer_transaction_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .customer
        .link_customer_transaction(998, &"45-as456777hhhhhhhhhh77777777-324".to_string(), None)
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/Customer/link/998/45-as456777hhhhhhhhhh77777777-324",
        None,
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_customer_request_consent_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client.customer.request_consent(998, None).await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("POST", "/Customer/998/consent", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_customer_update_customer_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .customer
        .update_customer(
            998,
            &CustomerData {
                customer_number: None,
                customer_username: None,
                customer_psw: None,
                customer_status: None,
                company: None,
                firstname: Some("Irene".to_string()),
                lastname: Some("Canizales".to_string()),
                phone: None,
                email: None,
                address: None,
                address_1: Some("145 Bishop's Trail".to_string()),
                city: Some("Mountain City".to_string()),
                state: Some("TN".to_string()),
                zip: Some("37612".to_string()),
                country: Some("US".to_string()),
                shipping_address: None,
                shipping_address_1: None,
                shipping_city: None,
                shipping_state: None,
                shipping_zip: None,
                shipping_country: None,
                balance: None,
                time_zone: None,
                additional_fields: None,
                identifier_fields: None,
                created_at: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("PUT", "/Customer/998", None, 1)
        .await
        .unwrap();
}
