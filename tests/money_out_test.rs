use payabli_api::prelude::*;

mod wire_test_utils;

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_money_out_authorize_out_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .money_out
        .authorize_out(
            &AuthorizeOutRequest {
                body: AuthorizePayoutBody {
                    entry_point: Entrypointfield("48acde49".to_string()),
                    order_description: Some(Orderdescription("Window Painting".to_string())),
                    payment_method: AuthorizePaymentMethod {
                        method: "managed".to_string(),
                        ..Default::default()
                    },
                    payment_details: RequestOutAuthorizePaymentDetails {
                        total_amount: Some(47.0),
                        unbundled: Some(false),
                        ..Default::default()
                    },
                    vendor_data: RequestOutAuthorizeVendorData {
                        vendor_number: Some(VendorNumber("7895433".to_string())),
                        ..Default::default()
                    },
                    invoice_data: vec![RequestOutAuthorizeInvoiceData {
                        bill_id: Some(BillId(54323)),
                        ..Default::default()
                    }],
                    auto_capture: Some(AutoCapture(true)),
                    ..Default::default()
                },
                allow_duplicated_bills: None,
                do_not_create_bills: None,
                force_vendor_creation: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("POST", "/MoneyOut/authorize", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_money_out_cancel_all_out_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .money_out
        .cancel_all_out(
            &vec!["2-29".to_string(), "2-28".to_string(), "2-27".to_string()],
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("POST", "/MoneyOut/cancelAll", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_money_out_cancel_out_get_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .money_out
        .cancel_out_get(&"129-219".to_string(), None)
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("GET", "/MoneyOut/cancel/129-219", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_money_out_cancel_out_delete_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .money_out
        .cancel_out_delete(&"129-219".to_string(), None)
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("DELETE", "/MoneyOut/cancel/129-219", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_money_out_capture_all_out_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .money_out
        .capture_all_out(
            &vec!["2-29".to_string(), "2-28".to_string(), "2-27".to_string()],
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("POST", "/MoneyOut/captureAll", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_money_out_capture_out_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .money_out
        .capture_out(&"129-219".to_string(), None)
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("GET", "/MoneyOut/capture/129-219", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_money_out_payout_details_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .money_out
        .payout_details(&"45-as456777hhhhhhhhhh77777777-324".to_string(), None)
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/MoneyOut/details/45-as456777hhhhhhhhhh77777777-324",
        None,
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_money_out_v_card_get_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .money_out
        .v_card_get(&"20230403315245421165".to_string(), None)
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("GET", "/MoneyOut/vcard/20230403315245421165", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_money_out_send_v_card_link_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .money_out
        .send_v_card_link(
            &SendVCardLinkRequest {
                trans_id: "01K33Z6YQZ6GD5QVKZ856MJBSC".to_string(),
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("POST", "/vcard/send-card-link", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_money_out_get_check_image_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .money_out
        .get_check_image(
            &"check133832686289732320_01JKBNZ5P32JPTZY8XXXX000000.pdf".to_string(),
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/MoneyOut/checkimage/check133832686289732320_01JKBNZ5P32JPTZY8XXXX000000.pdf",
        None,
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_money_out_update_check_payment_status_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .money_out
        .update_check_payment_status(
            &"TRANS123456".to_string(),
            &AllowedCheckPaymentStatus::Paid,
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("PATCH", "/MoneyOut/status/TRANS123456/5", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_money_out_reissue_out_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .money_out
        .reissue_out(
            &ReissueOutRequest {
                trans_id: "129-219".to_string(),
                body: ReissuePayoutBody {
                    payment_method: ReissuePaymentMethod {
                        method: "ach".to_string(),
                        ach_holder: Some("Acme Corp".to_string()),
                        ach_routing: Some("021000021".to_string()),
                        ach_account: Some("9876543210".to_string()),
                        ach_account_type: Some("savings".to_string()),
                        ach_holder_type: Some(AchHolderType::Business),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "POST",
        "/MoneyOut/reissue",
        Some(HashMap::from([("transId".to_string(), json!("129-219"))])),
        1,
    )
    .await
    .unwrap();
}
