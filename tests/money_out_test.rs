use payabli_api::prelude::*;

mod wire_test_utils;

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_money_out_authorize_out_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

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
                    source: None,
                    order_id: None,
                    order_description: Some(Orderdescription("Window Painting".to_string())),
                    payment_method: AuthorizePaymentMethod {
                        method: "managed".to_string(),
                        ach_holder: None,
                        ach_routing: None,
                        ach_account: None,
                        ach_account_type: None,
                        ach_code: None,
                        ach_holder_type: None,
                        stored_method_id: None,
                        initiator: None,
                        stored_method_usage_type: None,
                    },
                    payment_details: RequestOutAuthorizePaymentDetails {
                        check_number: None,
                        currency: None,
                        service_fee: None,
                        total_amount: Some(47.0),
                        unbundled: Some(false),
                    },
                    vendor_data: RequestOutAuthorizeVendorData {
                        vendor_number: Some(VendorNumber("7895433".to_string())),
                        name_1: None,
                        name_2: None,
                        ein: None,
                        phone: None,
                        email: None,
                        address_1: None,
                        city: None,
                        state: None,
                        zip: None,
                        country: None,
                        mcc: None,
                        contacts: None,
                        billing_data: None,
                        vendor_status: None,
                        remit_address_1: None,
                        remit_address_2: None,
                        remit_city: None,
                        remit_state: None,
                        remit_zip: None,
                        remit_country: None,
                        customer_vendor_account: None,
                        custom_field_1: None,
                        custom_field_2: None,
                        additional_data: None,
                        address_2: None,
                        internal_reference_id: None,
                        location_code: None,
                        payee_name_1: None,
                        payee_name_2: None,
                        payment_method: None,
                        vendor_id: None,
                    },
                    invoice_data: vec![RequestOutAuthorizeInvoiceData {
                        invoice_number: None,
                        net_amount: None,
                        invoice_date: None,
                        due_date: None,
                        comments: None,
                        lot_number: None,
                        bill_id: Some(BillId(54323)),
                        discount: None,
                        terms: None,
                        accounting_field_1: None,
                        accounting_field_2: None,
                        additional_data: None,
                        attachments: None,
                    }],
                    account_id: None,
                    subdomain: None,
                    subscription_id: None,
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
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

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
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

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
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

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
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

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
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

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
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

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
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

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
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

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
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

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
