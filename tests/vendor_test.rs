use payabli_api::prelude::*;

mod wire_test_utils;

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_vendor_add_vendor_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .vendor
        .add_vendor(
            &"8cfec329267".to_string(),
            &VendorData {
                vendor_number: Some(VendorNumber("VEN-123".to_string())),
                address_1: Some(AddressNullable("123 Ocean Drive".to_string())),
                address_2: Some(AddressAddtlNullable("Suite 400".to_string())),
                billing_data: Some(BillingData {
                    account_number: Some("123123123".to_string()),
                    bank_account_function: Some(0),
                    bank_account_holder_name: Some(BankAccountHolderName(
                        "Gruzya Adventure Outfitters LLC".to_string(),
                    )),
                    bank_account_holder_type: Some(BankAccountHolderType::Business),
                    bank_name: Some(BankName("Country Bank".to_string())),
                    id: Some(123),
                    routing_account: Some(RoutingAccount("123123123".to_string())),
                    type_account: Some(TypeAccount::Checking),
                    ..Default::default()
                }),
                city: Some("Miami".to_string()),
                contacts: Some(ContactsField(vec![Contacts {
                    contact_email: Some(Email("example@email.com".to_string())),
                    contact_name: Some("Herman Martinez".to_string()),
                    contact_phone: Some("3055550000".to_string()),
                    contact_title: Some("Owner".to_string()),
                    ..Default::default()
                }])),
                country: Some("US".to_string()),
                customer_vendor_account: Some("A-37622".to_string()),
                ein: Some(VendorEin("12-3456789".to_string())),
                email: Some(Email("example@email.com".to_string())),
                internal_reference_id: Some(123),
                location_code: Some(LocationCode("MIA123".to_string())),
                mcc: Some(Mcc("7777".to_string())),
                name_1: Some(VendorName1("Herman's Coatings and Masonry".to_string())),
                name_2: Some(VendorName2("<string>".to_string())),
                payee_name_1: Some(PayeeName("<string>".to_string())),
                payee_name_2: Some(PayeeName("<string>".to_string())),
                payment_method: Some(VendorPaymentMethodString("managed".to_string())),
                phone: Some(VendorPhone("5555555555".to_string())),
                remit_address_1: Some(Remitaddress1("123 Walnut Street".to_string())),
                remit_address_2: Some(Remitaddress2("Suite 900".to_string())),
                remit_city: Some(Remitcity("Miami".to_string())),
                remit_country: Some(Remitcountry("US".to_string())),
                remit_state: Some(Remitstate("FL".to_string())),
                remit_zip: Some(Remitzip("31113".to_string())),
                state: Some("FL".to_string()),
                vendor_status: Some(Vendorstatus(1)),
                zip: Some("33139".to_string()),
                ..Default::default()
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("POST", "/Vendor/single/8cfec329267", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_vendor_get_vendor_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client.vendor.get_vendor(1, None).await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("GET", "/Vendor/1", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_vendor_edit_vendor_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .vendor
        .edit_vendor(
            1,
            &VendorData {
                name_1: Some(VendorName1("Theodore's Janitorial".to_string())),
                ..Default::default()
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("PUT", "/Vendor/1", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_vendor_delete_vendor_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client.vendor.delete_vendor(1, None).await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("DELETE", "/Vendor/1", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_vendor_enrich_vendor_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .vendor
        .enrich_vendor(
            &"8cfec329267".to_string(),
            &VendorEnrichRequest {
                vendor_id: 456,
                scope: Some(vec!["invoice_scan".to_string()]),
                apply_enrichment_data: Some(false),
                invoice_file: Some(FileContent {
                    f_content: Some("<base64-encoded-pdf>".to_string()),
                    filename: Some("invoice-2026-001.pdf".to_string()),
                    ftype: Some(FileContentFtype::Pdf),
                    ..Default::default()
                }),
                fallback_method: Some("check".to_string()),
                schedule_call_if_needed: None,
                bill_id: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("POST", "/Vendor/enrich/8cfec329267", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_vendor_schedule_enrichment_call_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .vendor
        .schedule_enrichment_call(
            &"8cfec329267".to_string(),
            &ScheduleEnrichmentCallRequest {
                vendor_id: 456,
                phone: Some("5555550200".to_string()),
                enrichment_id: Some("enrich-3890-a1b2c3d4".to_string()),
                bill_id: Some(54323),
                fallback_method: Some("check".to_string()),
                max_retries: Some(3),
                timezone: Some("America/New_York".to_string()),
                send_now: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "POST",
        "/Vendor/enrich/schedule_call/8cfec329267",
        None,
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_vendor_get_enrichment_call_status_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client.vendor.get_enrichment_call_status(456, None).await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("GET", "/Vendor/456/enrichment/call-status", None, 1)
        .await
        .unwrap();
}
