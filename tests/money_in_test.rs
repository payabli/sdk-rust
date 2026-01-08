use payabli_api::prelude::*;

mod wire_test_utils;

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_money_in_authorize_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .money_in
        .authorize(
            &AuthorizeRequest {
                body: TransRequestBody {
                    account_id: None,
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
                    ipaddress: Some(IpAddress("255.255.255.255".to_string())),
                    order_description: None,
                    order_id: None,
                    payment_details: PaymentDetail {
                        categories: None,
                        check_image: None,
                        check_number: None,
                        currency: None,
                        service_fee: Some(0.0),
                        split_funding: None,
                        total_amount: 100.0,
                    },
                    payment_method: PaymentMethod::PayMethodCredit(PayMethodCredit {
                        cardcvv: Some(Cardcvv("999".to_string())),
                        cardexp: Cardexp("02/27".to_string()),
                        card_holder: Some(Cardholder("John Cassian".to_string())),
                        cardnumber: Cardnumber("4111111111111111".to_string()),
                        cardzip: Some(Cardzip("12345".to_string())),
                        initiator: Some(Initiator("payor".to_string())),
                        method: "card".to_string(),
                        save_if_success: None,
                    }),
                    source: None,
                    subdomain: None,
                    subscription_id: None,
                },
                force_customer_creation: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("POST", "/MoneyIn/authorize", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_money_in_capture_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .money_in
        .capture(
            &"10-7d9cd67d-2d5d-4cd7-a1b7-72b8b201ec13".to_string(),
            0.0,
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/MoneyIn/capture/10-7d9cd67d-2d5d-4cd7-a1b7-72b8b201ec13/0",
        None,
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_money_in_capture_auth_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .money_in
        .capture_auth(
            &"10-7d9cd67d-2d5d-4cd7-a1b7-72b8b201ec13".to_string(),
            &CaptureRequest {
                payment_details: CapturePaymentDetails {
                    total_amount: 105.0,
                    service_fee: Some(5.0),
                },
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "POST",
        "/MoneyIn/capture/10-7d9cd67d-2d5d-4cd7-a1b7-72b8b201ec13",
        None,
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_money_in_credit_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .money_in
        .credit(
            &RequestCredit {
                customer_data: PayorDataRequest {
                    additional_data: None,
                    billing_address_1: Some(BillingAddressNullable(
                        "5127 Linkwood ave".to_string(),
                    )),
                    billing_address_2: None,
                    billing_city: None,
                    billing_country: None,
                    billing_email: None,
                    billing_phone: None,
                    billing_state: None,
                    billing_zip: None,
                    company: None,
                    customer_id: None,
                    customer_number: Some(CustomerNumberNullable("100".to_string())),
                    first_name: None,
                    identifier_fields: None,
                    last_name: None,
                    shipping_address_1: None,
                    shipping_address_2: None,
                    shipping_city: None,
                    shipping_country: None,
                    shipping_state: None,
                    shipping_zip: None,
                },
                entrypoint: Some(Entrypointfield("my-entrypoint".to_string())),
                payment_details: PaymentDetailCredit {
                    currency: None,
                    service_fee: Some(0.0),
                    total_amount: 1.0,
                },
                payment_method: RequestCreditPaymentMethod {
                    ach_account: Some(Achaccount("88354454".to_string())),
                    ach_account_type: Some(Achaccounttype::Checking),
                    ach_code: None,
                    ach_holder: Some(AchHolder("John Smith".to_string())),
                    ach_routing: Some(Achrouting("021000021".to_string())),
                    method: "ach".to_string(),
                },
                force_customer_creation: None,
                account_id: None,
                order_description: None,
                order_id: None,
                source: None,
                subdomain: None,
            },
            Some(
                RequestOptions::new()
                    .additional_header("idempotencyKey", "6B29FC40-CA47-1067-B31D-00DD010662DA"),
            ),
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("POST", "/MoneyIn/makecredit", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_money_in_details_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .money_in
        .details(&"45-as456777hhhhhhhhhh77777777-324".to_string(), None)
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/MoneyIn/details/45-as456777hhhhhhhhhh77777777-324",
        None,
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_money_in_getpaid_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .money_in
        .getpaid(
            &GetpaidRequest {
                body: TransRequestBody {
                    account_id: None,
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
                    ipaddress: Some(IpAddress("255.255.255.255".to_string())),
                    order_description: None,
                    order_id: None,
                    payment_details: PaymentDetail {
                        categories: None,
                        check_image: None,
                        check_number: None,
                        currency: None,
                        service_fee: Some(0.0),
                        split_funding: None,
                        total_amount: 100.0,
                    },
                    payment_method: PaymentMethod::PayMethodCredit(PayMethodCredit {
                        cardcvv: Some(Cardcvv("999".to_string())),
                        cardexp: Cardexp("02/27".to_string()),
                        card_holder: Some(Cardholder("John Cassian".to_string())),
                        cardnumber: Cardnumber("4111111111111111".to_string()),
                        cardzip: Some(Cardzip("12345".to_string())),
                        initiator: Some(Initiator("payor".to_string())),
                        method: "card".to_string(),
                        save_if_success: None,
                    }),
                    source: None,
                    subdomain: None,
                    subscription_id: None,
                },
                ach_validation: None,
                force_customer_creation: None,
                include_details: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("POST", "/MoneyIn/getpaid", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_money_in_reverse_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .money_in
        .reverse(
            &"10-3ffa27df-b171-44e0-b251-e95fbfc7a723".to_string(),
            0.0,
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/MoneyIn/reverse/10-3ffa27df-b171-44e0-b251-e95fbfc7a723/0",
        None,
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_money_in_refund_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .money_in
        .refund(
            &"10-3ffa27df-b171-44e0-b251-e95fbfc7a723".to_string(),
            0.0,
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/MoneyIn/refund/10-3ffa27df-b171-44e0-b251-e95fbfc7a723/0",
        None,
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_money_in_refund_with_instructions_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .money_in
        .refund_with_instructions(
            &"10-3ffa27df-b171-44e0-b251-e95fbfc7a723".to_string(),
            &RequestRefund {
                amount: Some(100.0),
                order_description: Some(Orderdescription("Materials deposit".to_string())),
                refund_details: Some(RefundDetail {
                    categories: None,
                    split_refunding: Some(vec![
                        SplitFundingRefundContent {
                            account_id: Some("187-342".to_string()),
                            amount: Some(60.0),
                            description: Some("Refunding undelivered materials".to_string()),
                            origination_entry_point: Some("7f1a381696".to_string()),
                        },
                        SplitFundingRefundContent {
                            account_id: Some("187-343".to_string()),
                            amount: Some(40.0),
                            description: Some(
                                "Refunding deposit for undelivered materials".to_string(),
                            ),
                            origination_entry_point: Some("7f1a381696".to_string()),
                        },
                    ]),
                }),
                source: Some(Source("api".to_string())),
                ipaddress: None,
                order_id: None,
            },
            Some(
                RequestOptions::new()
                    .additional_header("idempotencyKey", "8A29FC40-CA47-1067-B31D-00DD010662DB"),
            ),
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "POST",
        "/MoneyIn/refund/10-3ffa27df-b171-44e0-b251-e95fbfc7a723",
        None,
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_money_in_reverse_credit_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .money_in
        .reverse_credit(&"45-as456777hhhhhhhhhh77777777-324".to_string(), None)
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/MoneyIn/reverseCredit/45-as456777hhhhhhhhhh77777777-324",
        None,
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_money_in_send_receipt_2_trans_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .money_in
        .send_receipt_2_trans(
            &"45-as456777hhhhhhhhhh77777777-324".to_string(),
            &SendReceipt2TransQueryRequest {
                email: Some("example@email.com".to_string()),
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/MoneyIn/sendreceipt/45-as456777hhhhhhhhhh77777777-324",
        Some(HashMap::from([(
            "email".to_string(),
            "example@email.com".to_string(),
        )])),
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_money_in_validate_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .money_in
        .validate(
            &RequestPaymentValidate {
                entry_point: Entrypointfield("entry132".to_string()),
                payment_method: RequestPaymentValidatePaymentMethod {
                    method: RequestPaymentValidatePaymentMethodMethod::Card,
                    cardnumber: Cardnumber("4360000001000005".to_string()),
                    cardexp: Cardexp("12/29".to_string()),
                    cardzip: Cardzip("14602-8328".to_string()),
                    card_holder: Cardholder("Dianne Becker-Smith".to_string()),
                },
                account_id: None,
                order_description: None,
                order_id: None,
            },
            Some(
                RequestOptions::new()
                    .additional_header("idempotencyKey", "6B29FC40-CA47-1067-B31D-00DD010662DA"),
            ),
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("POST", "/MoneyIn/validate", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_money_in_void_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .money_in
        .void(&"10-3ffa27df-b171-44e0-b251-e95fbfc7a723".to_string(), None)
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/MoneyIn/void/10-3ffa27df-b171-44e0-b251-e95fbfc7a723",
        None,
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_money_in_getpaidv_2_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .money_in
        .getpaidv_2(
            &Getpaidv2Request {
                body: TransRequestBody {
                    account_id: None,
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
                    ipaddress: Some(IpAddress("255.255.255.255".to_string())),
                    order_description: None,
                    order_id: None,
                    payment_details: PaymentDetail {
                        categories: None,
                        check_image: None,
                        check_number: None,
                        currency: None,
                        service_fee: Some(0.0),
                        split_funding: None,
                        total_amount: 100.0,
                    },
                    payment_method: PaymentMethod::PayMethodCredit(PayMethodCredit {
                        cardcvv: Some(Cardcvv("999".to_string())),
                        cardexp: Cardexp("02/27".to_string()),
                        card_holder: Some(Cardholder("John Cassian".to_string())),
                        cardnumber: Cardnumber("4111111111111111".to_string()),
                        cardzip: Some(Cardzip("12345".to_string())),
                        initiator: Some(Initiator("payor".to_string())),
                        method: "card".to_string(),
                        save_if_success: None,
                    }),
                    source: None,
                    subdomain: None,
                    subscription_id: None,
                },
                ach_validation: None,
                force_customer_creation: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("POST", "/v2/MoneyIn/getpaid", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_money_in_authorizev_2_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .money_in
        .authorizev_2(
            &Authorizev2Request {
                body: TransRequestBody {
                    account_id: None,
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
                    ipaddress: Some(IpAddress("255.255.255.255".to_string())),
                    order_description: None,
                    order_id: None,
                    payment_details: PaymentDetail {
                        categories: None,
                        check_image: None,
                        check_number: None,
                        currency: None,
                        service_fee: Some(0.0),
                        split_funding: None,
                        total_amount: 100.0,
                    },
                    payment_method: PaymentMethod::PayMethodCredit(PayMethodCredit {
                        cardcvv: Some(Cardcvv("999".to_string())),
                        cardexp: Cardexp("02/27".to_string()),
                        card_holder: Some(Cardholder("John Cassian".to_string())),
                        cardnumber: Cardnumber("4111111111111111".to_string()),
                        cardzip: Some(Cardzip("12345".to_string())),
                        initiator: Some(Initiator("payor".to_string())),
                        method: "card".to_string(),
                        save_if_success: None,
                    }),
                    source: None,
                    subdomain: None,
                    subscription_id: None,
                },
                force_customer_creation: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("POST", "/v2/MoneyIn/authorize", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_money_in_capturev_2_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .money_in
        .capturev_2(
            &"10-7d9cd67d-2d5d-4cd7-a1b7-72b8b201ec13".to_string(),
            &CaptureRequest {
                payment_details: CapturePaymentDetails {
                    total_amount: 105.0,
                    service_fee: Some(5.0),
                },
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "POST",
        "/v2/MoneyIn/capture/10-7d9cd67d-2d5d-4cd7-a1b7-72b8b201ec13",
        None,
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_money_in_refundv_2_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .money_in
        .refundv_2(&"10-3ffa27df-b171-44e0-b251-e95fbfc7a723".to_string(), None)
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "POST",
        "/v2/MoneyIn/refund/10-3ffa27df-b171-44e0-b251-e95fbfc7a723",
        None,
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_money_in_refundv_2_amount_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .money_in
        .refundv_2_amount(
            &"10-3ffa27df-b171-44e0-b251-e95fbfc7a723".to_string(),
            0.0,
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "POST",
        "/v2/MoneyIn/refund/10-3ffa27df-b171-44e0-b251-e95fbfc7a723/0",
        None,
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_money_in_voidv_2_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .money_in
        .voidv_2(&"10-3ffa27df-b171-44e0-b251-e95fbfc7a723".to_string(), None)
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "POST",
        "/v2/MoneyIn/void/10-3ffa27df-b171-44e0-b251-e95fbfc7a723",
        None,
        1,
    )
    .await
    .unwrap();
}
