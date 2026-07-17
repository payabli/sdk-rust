use payabli_api::prelude::*;

mod wire_test_utils;

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_bill_add_bill_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
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
                attachments: Some(Attachments(vec![FileContent {
                    filename: Some("my-doc.pdf".to_string()),
                    ftype: Some(FileContentFtype::Pdf),
                    furl: Some("https://mysite.com/my-doc.pdf".to_string()),
                    ..Default::default()
                }])),
                bill_date: Some(NaiveDate::parse_from_str("2024-07-01", "%Y-%m-%d").unwrap()),
                bill_items: Some(Billitems(vec![BillItem {
                    item_categories: Some(vec!["deposits".to_string()]),
                    item_commodity_code: Some(ItemCommodityCode("010".to_string())),
                    item_cost: Some(5.0),
                    item_description: Some(ItemDescription("Deposit for materials".to_string())),
                    item_mode: Some(0),
                    item_product_code: Some(ItemProductCode("M-DEPOSIT".to_string())),
                    item_product_name: Some(ItemProductName("Materials deposit".to_string())),
                    item_qty: Some(1),
                    item_tax_amount: Some(7.0),
                    item_tax_rate: Some(0.075),
                    item_total_amount: Some(123.0),
                    item_unit_of_measure: Some(ItemUnitofMeasure("SqFt".to_string())),
                    ..Default::default()
                }])),
                bill_number: Some("ABC-123".to_string()),
                comments: Some(Comments("Deposit for materials".to_string())),
                due_date: Some(NaiveDate::parse_from_str("2024-07-01", "%Y-%m-%d").unwrap()),
                end_date: Some(NaiveDate::parse_from_str("2024-07-01", "%Y-%m-%d").unwrap()),
                frequency: Some(Frequency::Monthly),
                mode: Some(0),
                net_amount: Some(3762.87),
                status: Some(Billstatus(1)),
                terms: Some(Terms::Net30),
                vendor: Some(BillOutDataVendor {
                    vendor_number: Some(VendorNumber("VEN-123".to_string())),
                    ..Default::default()
                }),
                ..Default::default()
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
async fn test_bill_get_bill_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
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
async fn test_bill_edit_bill_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .bill
        .edit_bill(
            285,
            &BillOutData {
                bill_date: Some(NaiveDate::parse_from_str("2025-07-01", "%Y-%m-%d").unwrap()),
                net_amount: Some(3762.87),
                ..Default::default()
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
async fn test_bill_delete_bill_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
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
async fn test_bill_get_attached_from_bill_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
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
                ..Default::default()
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/Bill/attachedFileFromBill/285/0_Bill.pdf",
        Some(HashMap::from([("returnObject".to_string(), json!("true"))])),
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_bill_delete_attached_from_bill_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
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
                ..Default::default()
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
async fn test_bill_send_to_approval_bill_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .bill
        .send_to_approval_bill(
            285,
            &SendToApprovalBillRequest {
                body: vec!["approver@example.com".to_string()],
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
async fn test_bill_modify_approval_bill_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .bill
        .modify_approval_bill(
            285,
            &vec![
                "approver1@example.com".to_string(),
                "approver2@example.com".to_string(),
            ],
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("PUT", "/Bill/approval/285", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_bill_set_approved_bill_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .bill
        .set_approved_bill(
            285,
            &"true".to_string(),
            &SetApprovedBillQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("GET", "/Bill/approval/285/true", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_bill_list_bills_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
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
                ..Default::default()
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/Query/bills/8cfec329267",
        Some(HashMap::from([
            ("fromRecord".to_string(), json!("251")),
            ("limitRecord".to_string(), json!("0")),
            ("sortBy".to_string(), json!("desc(field_name)")),
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
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
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
                ..Default::default()
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/Query/bills/org/123",
        Some(HashMap::from([
            ("fromRecord".to_string(), json!("251")),
            ("limitRecord".to_string(), json!("0")),
            ("sortBy".to_string(), json!("desc(field_name)")),
        ])),
        1,
    )
    .await
    .unwrap();
}
