use payabli_api::prelude::*;

mod wire_test_utils;

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_vendor_add_vendor_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .vendor
        .add_vendor(
            &"8cfec329267".to_string(),
            &VendorData {
                vendor_number: Some(VendorNumber("1234".to_string())),
                additional_data: None,
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
                }),
                city: Some("Miami".to_string()),
                contacts: Some(ContactsField(Some(vec![Contacts {
                    contact_email: Some(Email("example@email.com".to_string())),
                    contact_name: Some("Herman Martinez".to_string()),
                    contact_phone: Some("3055550000".to_string()),
                    contact_title: Some("Owner".to_string()),
                    additional_data: None,
                }]))),
                country: Some("US".to_string()),
                custom_field_1: None,
                custom_field_2: None,
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
                phone: Some(VendorPhone(Some("5555555555".to_string()))),
                remit_address_1: Some(Remitaddress1("123 Walnut Street".to_string())),
                remit_address_2: Some(Remitaddress2("Suite 900".to_string())),
                remit_city: Some(Remitcity("Miami".to_string())),
                remit_country: Some(Remitcountry("US".to_string())),
                remit_email: None,
                remit_state: Some(Remitstate("FL".to_string())),
                remit_zip: Some(Remitzip("31113".to_string())),
                state: Some("FL".to_string()),
                vendor_status: Some(Vendorstatus(1)),
                zip: Some("33139".to_string()),
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
async fn test_vendor_delete_vendor_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
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
async fn test_vendor_edit_vendor_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .vendor
        .edit_vendor(
            1,
            &VendorData {
                vendor_number: None,
                additional_data: None,
                address_1: None,
                address_2: None,
                billing_data: None,
                city: None,
                contacts: None,
                country: None,
                custom_field_1: None,
                custom_field_2: None,
                customer_vendor_account: None,
                ein: None,
                email: None,
                internal_reference_id: None,
                location_code: None,
                mcc: None,
                name_1: Some(VendorName1("Theodore's Janitorial".to_string())),
                name_2: None,
                payee_name_1: None,
                payee_name_2: None,
                payment_method: None,
                phone: None,
                remit_address_1: None,
                remit_address_2: None,
                remit_city: None,
                remit_country: None,
                remit_email: None,
                remit_state: None,
                remit_zip: None,
                state: None,
                vendor_status: None,
                zip: None,
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
async fn test_vendor_get_vendor_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
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
