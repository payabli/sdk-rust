use payabli_api::prelude::*;

mod wire_test_utils;

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_invoice_add_invoice_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .invoice
        .add_invoice(
            &"8cfec329267".to_string(),
            &AddInvoiceRequest {
                body: InvoiceDataRequest {
                    customer_data: Some(PayorDataRequest {
                        customer_number: Some(CustomerNumberNullable("C-90010".to_string())),
                        first_name: Some("Tamara".to_string()),
                        last_name: Some("Bagratoni".to_string()),
                        ..Default::default()
                    }),
                    invoice_data: Some(BillData {
                        discount: Some(Discount(10.0)),
                        frequency: Some(Frequency::OneTime),
                        invoice_amount: Some(InvoiceAmount(1082.37)),
                        invoice_date: Some(
                            NaiveDate::parse_from_str("2025-10-19", "%Y-%m-%d").unwrap(),
                        ),
                        invoice_number: Some(InvoiceNumber("INV-2345".to_string())),
                        invoice_status: Some(Invoicestatus(1)),
                        invoice_type: Some(InvoiceType(0)),
                        items: Some(vec![
                            BillItem {
                                item_cost: Some(100.0),
                                item_description: Some(ItemDescription(
                                    "Consultation for Georgian tours".to_string(),
                                )),
                                item_mode: Some(2),
                                item_product_name: Some(ItemProductName(
                                    "Adventure Consult".to_string(),
                                )),
                                item_qty: Some(2),
                                item_total_amount: Some(200.0),
                                ..Default::default()
                            },
                            BillItem {
                                item_cost: Some(882.37),
                                item_description: Some(ItemDescription(
                                    "Deposit for trip planning".to_string(),
                                )),
                                item_mode: Some(2),
                                item_product_name: Some(ItemProductName("Deposit ".to_string())),
                                item_qty: Some(1),
                                item_total_amount: Some(882.37),
                                ..Default::default()
                            },
                        ]),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                force_customer_creation: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("POST", "/Invoice/8cfec329267", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_invoice_get_attached_file_from_invoice_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .invoice
        .get_attached_file_from_invoice(
            1,
            &"filename".to_string(),
            &GetAttachedFileFromInvoiceQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/Invoice/attachedFileFromInvoice/1/filename",
        None,
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_invoice_delete_attached_from_invoice_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .invoice
        .delete_attached_from_invoice(23548884, &"0_Bill.pdf".to_string(), None)
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "DELETE",
        "/Invoice/attachedFileFromInvoice/23548884/0_Bill.pdf",
        None,
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_invoice_get_invoice_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client.invoice.get_invoice(23548884, None).await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("GET", "/Invoice/23548884", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_invoice_edit_invoice_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .invoice
        .edit_invoice(
            23548884,
            &EditInvoiceRequest {
                body: InvoiceDataRequest {
                    invoice_data: Some(BillData {
                        invoice_amount: Some(InvoiceAmount(982.37)),
                        invoice_date: Some(
                            NaiveDate::parse_from_str("2025-10-19", "%Y-%m-%d").unwrap(),
                        ),
                        invoice_number: Some(InvoiceNumber("INV-2345".to_string())),
                        items: Some(vec![BillItem {
                            item_cost: Some(882.37),
                            item_description: Some(ItemDescription(
                                "Deposit for trip planning".to_string(),
                            )),
                            item_product_name: Some(ItemProductName("Deposit".to_string())),
                            item_qty: Some(1),
                            ..Default::default()
                        }]),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                force_customer_creation: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("PUT", "/Invoice/23548884", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_invoice_delete_invoice_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client.invoice.delete_invoice(23548884, None).await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("DELETE", "/Invoice/23548884", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_invoice_get_invoice_number_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .invoice
        .get_invoice_number(&"8cfec329267".to_string(), None)
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("GET", "/Invoice/getNumber/8cfec329267", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_invoice_list_invoices_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .invoice
        .list_invoices(
            &"8cfec329267".to_string(),
            &ListInvoicesQueryRequest {
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
        "/Query/invoices/8cfec329267",
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
async fn test_invoice_list_invoices_org_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .invoice
        .list_invoices_org(
            123,
            &ListInvoicesOrgQueryRequest {
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
        "/Query/invoices/org/123",
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
async fn test_invoice_send_invoice_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .invoice
        .send_invoice(
            23548884,
            &SendInvoiceQueryRequest {
                attachfile: Some(true),
                mail_2: Some("tamara@example.com".to_string()),
                ..Default::default()
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/Invoice/send/23548884",
        Some(HashMap::from([
            ("attachfile".to_string(), json!("true")),
            ("mail2".to_string(), json!("tamara@example.com")),
        ])),
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_invoice_get_invoice_pdf_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client.invoice.get_invoice_pdf(23548884, None).await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("GET", "/Export/invoicePdf/23548884", None, 1)
        .await
        .unwrap();
}
