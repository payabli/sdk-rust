use payabli_api::prelude::*;

mod wire_test_utils;

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_subscription_get_subscription_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client.subscription.get_subscription(263, None).await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("GET", "/Subscription/263", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_subscription_new_subscription_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .subscription
        .new_subscription(
            &NewSubscriptionRequest {
                body: SubscriptionRequestBody {
                    customer_data: Some(PayorDataRequest {
                        additional_data: None,
                        billing_address_1: None,
                        billing_address_2: None,
                        billing_city: None,
                        billing_country: None,
                        billing_email: None,
                        billing_phone: None,
                        billing_state: None,
                        billing_zip: None,
                        company: None,
                        customer_id: Some(CustomerId(4440)),
                        customer_number: None,
                        first_name: None,
                        identifier_fields: None,
                        last_name: None,
                        shipping_address_1: None,
                        shipping_address_2: None,
                        shipping_city: None,
                        shipping_country: None,
                        shipping_state: None,
                        shipping_zip: None,
                    }),
                    entry_point: Some(Entrypointfield("f743aed24a".to_string())),
                    invoice_data: None,
                    payment_details: Some(PaymentDetail {
                        categories: None,
                        check_image: None,
                        check_number: None,
                        currency: None,
                        service_fee: Some(0.0),
                        split_funding: None,
                        total_amount: 100.0,
                    }),
                    payment_method: Some(RequestSchedulePaymentMethod::PayMethodCredit(
                        PayMethodCredit {
                            cardcvv: Some(Cardcvv("123".to_string())),
                            cardexp: Cardexp("02/25".to_string()),
                            card_holder: Some(Cardholder("John Cassian".to_string())),
                            cardnumber: Cardnumber("4111111111111111".to_string()),
                            cardzip: Some(Cardzip("37615".to_string())),
                            initiator: Some(Initiator("payor".to_string())),
                            method: "card".to_string(),
                            save_if_success: None,
                        },
                    )),
                    schedule_details: Some(ScheduleDetail {
                        end_date: Some("03-20-2025".to_string()),
                        frequency: Some(Frequency::Weekly),
                        plan_id: Some(1),
                        start_date: Some("09-20-2024".to_string()),
                    }),
                    set_pause: None,
                    source: None,
                    subdomain: None,
                },
                force_customer_creation: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("POST", "/Subscription/add", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_subscription_remove_subscription_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client.subscription.remove_subscription(396, None).await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("DELETE", "/Subscription/396", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_subscription_update_subscription_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .subscription
        .update_subscription(
            231,
            &RequestUpdateSchedule {
                set_pause: Some(SetPause(true)),
                payment_details: None,
                schedule_details: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("PUT", "/Subscription/231", None, 1)
        .await
        .unwrap();
}
