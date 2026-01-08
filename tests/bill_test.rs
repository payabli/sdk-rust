use payabli_api::prelude::*;

mod wire_test_utils;

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_bill_add_bill_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .bill
        .add_bill(
            &"8cfec329267".to_string(),
            &BillOutData {
                accounting_field_1: Some(AccountingField("MyInternalId".to_string())),
                accounting_field_2: None,
                additional_data: None,
                attachments: Some(Attachments(Some(vec![FileContent {
                    f_content: None,
                    filename: Some("my-doc.pdf".to_string()),
                    ftype: Some(FileContentFtype::Pdf),
                    furl: Some("https://mysite.com/my-doc.pdf".to_string()),
                }]))),
                bill_date: Some(Datenullable(Some(
                    NaiveDate::parse_from_str("2024-07-01", "%Y-%m-%d").unwrap(),
                ))),
                bill_items: Some(Billitems(Some(vec![BillItem {
                    item_categories: Some(vec![Some("deposits".to_string())]),
                    item_commodity_code: Some(ItemCommodityCode("010".to_string())),
                    item_cost: 5.0,
                    item_description: Some(ItemDescription("Deposit for materials".to_string())),
                    item_mode: Some(0),
                    item_product_code: Some(ItemProductCode("M-DEPOSIT".to_string())),
                    item_product_name: Some(ItemProductName("Materials deposit".to_string())),
                    item_qty: Some(1),
                    item_tax_amount: Some(7.0),
                    item_tax_rate: Some(0.075),
                    item_total_amount: Some(123.0),
                    item_unit_of_measure: Some(ItemUnitofMeasure("SqFt".to_string())),
                }]))),
                bill_number: Some("ABC-123".to_string()),
                comments: Some(Comments("Deposit for materials".to_string())),
                discount: None,
                due_date: Some(Datenullable(Some(
                    NaiveDate::parse_from_str("2024-07-01", "%Y-%m-%d").unwrap(),
                ))),
                end_date: Some(Datenullable(Some(
                    NaiveDate::parse_from_str("2024-07-01", "%Y-%m-%d").unwrap(),
                ))),
                frequency: Some(Frequency::Monthly),
                lot_number: None,
                mode: Some(0),
                net_amount: Some(3762.87),
                scheduled_options: None,
                status: Some(Billstatus(-99)),
                terms: Some(Terms("NET30".to_string())),
                total_amount: None,
                vendor: Some(VendorData {
                    vendor_number: Some(VendorNumber("1234-A".to_string())),
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
                    name_1: None,
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
                }),
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("POST", "/Bill/single/8cfec329267", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_bill_delete_attached_from_bill_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .bill
        .delete_attached_from_bill(
            285,
            &"0_Bill.pdf".to_string(),
            &DeleteAttachedFromBillQueryRequest {
                return_object: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "DELETE",
        "/Bill/attachedFileFromBill/285/0_Bill.pdf",
        None,
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_bill_delete_bill_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client.bill.delete_bill(285, None).await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("DELETE", "/Bill/285", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_bill_edit_bill_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .bill
        .edit_bill(
            285,
            &BillOutData {
                accounting_field_1: None,
                accounting_field_2: None,
                additional_data: None,
                attachments: None,
                bill_date: Some(Datenullable(Some(
                    NaiveDate::parse_from_str("2025-07-01", "%Y-%m-%d").unwrap(),
                ))),
                bill_items: None,
                bill_number: None,
                comments: None,
                discount: None,
                due_date: None,
                end_date: None,
                frequency: None,
                lot_number: None,
                mode: None,
                net_amount: Some(3762.87),
                scheduled_options: None,
                status: None,
                terms: None,
                total_amount: None,
                vendor: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("PUT", "/Bill/285", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_bill_get_attached_from_bill_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .bill
        .get_attached_from_bill(
            285,
            &"0_Bill.pdf".to_string(),
            &GetAttachedFromBillQueryRequest {
                return_object: Some(true),
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/Bill/attachedFileFromBill/285/0_Bill.pdf",
        Some(HashMap::from([(
            "returnObject".to_string(),
            "true".to_string(),
        )])),
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_bill_get_bill_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client.bill.get_bill(285, None).await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("GET", "/Bill/285", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_bill_list_bills_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .bill
        .list_bills(
            &"8cfec329267".to_string(),
            &ListBillsQueryRequest {
                from_record: Some(251),
                limit_record: Some(0),
                sort_by: Some("desc(field_name)".to_string()),
                export_format: None,
                parameters: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/Query/bills/8cfec329267",
        Some(HashMap::from([
            ("fromRecord".to_string(), "251".to_string()),
            ("limitRecord".to_string(), "0".to_string()),
            ("sortBy".to_string(), "desc(field_name)".to_string()),
        ])),
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_bill_list_bills_org_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .bill
        .list_bills_org(
            123,
            &ListBillsOrgQueryRequest {
                from_record: Some(251),
                limit_record: Some(0),
                sort_by: Some("desc(field_name)".to_string()),
                export_format: None,
                parameters: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/Query/bills/org/123",
        Some(HashMap::from([
            ("fromRecord".to_string(), "251".to_string()),
            ("limitRecord".to_string(), "0".to_string()),
            ("sortBy".to_string(), "desc(field_name)".to_string()),
        ])),
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_bill_modify_approval_bill_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .bill
        .modify_approval_bill(285, &vec!["string".to_string()], None)
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("PUT", "/Bill/approval/285", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_bill_send_to_approval_bill_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .bill
        .send_to_approval_bill(
            285,
            &SendToApprovalBillRequest {
                body: vec!["string".to_string()],
                autocreate_user: None,
            },
            Some(
                RequestOptions::new()
                    .additional_header("idempotencyKey", "6B29FC40-CA47-1067-B31D-00DD010662DA"),
            ),
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("POST", "/Bill/approval/285", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_bill_set_approved_bill_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .bill
        .set_approved_bill(
            285,
            &"true".to_string(),
            &SetApprovedBillQueryRequest { email: None },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("GET", "/Bill/approval/285/true", None, 1)
        .await
        .unwrap();
}
