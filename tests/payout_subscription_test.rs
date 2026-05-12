use payabli_api::prelude::*;

mod wire_test_utils;

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_payout_subscription_create_payout_subscription_with_wiremock() {
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
        .payout_subscription
        .create_payout_subscription(
            &PayoutSubscriptionRequestBody {
                entry_point: Entrypointfield("d193cf9a46".to_string()),
                payment_method: AuthorizePaymentMethod {
                    method: "ach".to_string(),
                    ach_holder: Some("Herman Coatings".to_string()),
                    ach_routing: Some("021000021".to_string()),
                    ach_account: Some("3453445666".to_string()),
                    ach_account_type: Some("checking".to_string()),
                    ..Default::default()
                },
                payment_details: Some(PayoutPaymentDetail {
                    total_amount: 500.0,
                    service_fee: Some(0.0),
                    currency: Some("USD".to_string()),
                    ..Default::default()
                }),
                vendor_data: RequestOutAuthorizeVendorData {
                    vendor_id: Some(Vendorid(1501)),
                    ..Default::default()
                },
                bill_data: Some(vec![BillPayOutDataRequest {
                    due_date: Some(NaiveDate::parse_from_str("2025-08-15", "%Y-%m-%d").unwrap()),
                    invoice_date: Some(
                        NaiveDate::parse_from_str("2025-08-01", "%Y-%m-%d").unwrap(),
                    ),
                    invoice_number: Some(InvoiceNumber("INV-5001".to_string())),
                    net_amount: Some(NetAmountstring("500".to_string())),
                    ..Default::default()
                }]),
                schedule_details: Some(PayoutScheduleDetail {
                    start_date: Some("09/01/2025".to_string()),
                    end_date: Some("09/01/2026".to_string()),
                    frequency: Some(Frequency::Monthly),
                    ..Default::default()
                }),
                ..Default::default()
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("POST", "/PayoutSubscription", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_payout_subscription_get_payout_subscription_with_wiremock() {
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
        .payout_subscription
        .get_payout_subscription(42, None)
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("GET", "/PayoutSubscription/42", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_payout_subscription_update_payout_subscription_with_wiremock() {
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
        .payout_subscription
        .update_payout_subscription(
            42,
            &UpdatePayoutSubscriptionBody {
                set_pause: Some(PayoutSetPause(true)),
                ..Default::default()
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("PUT", "/PayoutSubscription/42", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_payout_subscription_delete_payout_subscription_with_wiremock() {
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
        .payout_subscription
        .delete_payout_subscription(42, None)
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("DELETE", "/PayoutSubscription/42", None, 1)
        .await
        .unwrap();
}
