use payabli_api::prelude::*;

mod wire_test_utils;

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_invoice_add_invoice_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

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
                        customer_id: None,
                        customer_number: Some(CustomerNumberNullable("3".to_string())),
                        first_name: Some("Tamara".to_string()),
                        identifier_fields: None,
                        last_name: Some("Bagratoni".to_string()),
                        shipping_address_1: None,
                        shipping_address_2: None,
                        shipping_city: None,
                        shipping_country: None,
                        shipping_state: None,
                        shipping_zip: None,
                    }),
                    invoice_data: Some(BillData {
                        additional_data: None,
                        attachments: None,
                        company: None,
                        discount: Some(Discount(Some(10.0))),
                        duty_amount: None,
                        first_name: None,
                        freight_amount: None,
                        frequency: Some(Frequency::OneTime),
                        invoice_amount: Some(InvoiceAmount(982.37)),
                        invoice_date: Some(Datenullable(Some(
                            NaiveDate::parse_from_str("2025-10-19", "%Y-%m-%d").unwrap(),
                        ))),
                        invoice_due_date: None,
                        invoice_end_date: None,
                        invoice_number: Some(InvoiceNumber("INV-3".to_string())),
                        invoice_status: Some(Invoicestatus(1)),
                        invoice_type: Some(InvoiceType(0)),
                        items: Some(vec![
                            BillItem {
                                item_categories: None,
                                item_commodity_code: None,
                                item_cost: 100.0,
                                item_description: Some(ItemDescription(
                                    "Consultation for Georgian tours".to_string(),
                                )),
                                item_mode: Some(1),
                                item_product_code: None,
                                item_product_name: Some(ItemProductName(
                                    "Adventure Consult".to_string(),
                                )),
                                item_qty: Some(1),
                                item_tax_amount: None,
                                item_tax_rate: None,
                                item_total_amount: Some(1.0),
                                item_unit_of_measure: None,
                            },
                            BillItem {
                                item_categories: None,
                                item_commodity_code: None,
                                item_cost: 882.37,
                                item_description: Some(ItemDescription(
                                    "Deposit for trip planning".to_string(),
                                )),
                                item_mode: None,
                                item_product_code: None,
                                item_product_name: Some(ItemProductName("Deposit ".to_string())),
                                item_qty: Some(1),
                                item_tax_amount: None,
                                item_tax_rate: None,
                                item_total_amount: Some(1.0),
                                item_unit_of_measure: None,
                            },
                        ]),
                        last_name: None,
                        notes: None,
                        payment_terms: None,
                        purchase_order: None,
                        shipping_address_1: None,
                        shipping_address_2: None,
                        shipping_city: None,
                        shipping_country: None,
                        shipping_email: None,
                        shipping_from_zip: None,
                        shipping_phone: None,
                        shipping_state: None,
                        shipping_zip: None,
                        summary_commodity_code: None,
                        tax: None,
                        terms_conditions: None,
                    }),
                    scheduled_options: None,
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
async fn test_invoice_delete_attached_from_invoice_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

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
async fn test_invoice_delete_invoice_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

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
async fn test_invoice_edit_invoice_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .invoice
        .edit_invoice(
            332,
            &EditInvoiceRequest {
                body: InvoiceDataRequest {
                    customer_data: None,
                    invoice_data: Some(BillData {
                        additional_data: None,
                        attachments: None,
                        company: None,
                        discount: None,
                        duty_amount: None,
                        first_name: None,
                        freight_amount: None,
                        frequency: None,
                        invoice_amount: Some(InvoiceAmount(982.37)),
                        invoice_date: Some(Datenullable(Some(
                            NaiveDate::parse_from_str("2025-10-19", "%Y-%m-%d").unwrap(),
                        ))),
                        invoice_due_date: None,
                        invoice_end_date: None,
                        invoice_number: Some(InvoiceNumber("INV-6".to_string())),
                        invoice_status: None,
                        invoice_type: None,
                        items: Some(vec![BillItem {
                            item_categories: None,
                            item_commodity_code: None,
                            item_cost: 882.37,
                            item_description: Some(ItemDescription(
                                "Deposit for trip planning".to_string(),
                            )),
                            item_mode: None,
                            item_product_code: None,
                            item_product_name: Some(ItemProductName("Deposit".to_string())),
                            item_qty: Some(1),
                            item_tax_amount: None,
                            item_tax_rate: None,
                            item_total_amount: None,
                            item_unit_of_measure: None,
                        }]),
                        last_name: None,
                        notes: None,
                        payment_terms: None,
                        purchase_order: None,
                        shipping_address_1: None,
                        shipping_address_2: None,
                        shipping_city: None,
                        shipping_country: None,
                        shipping_email: None,
                        shipping_from_zip: None,
                        shipping_phone: None,
                        shipping_state: None,
                        shipping_zip: None,
                        summary_commodity_code: None,
                        tax: None,
                        terms_conditions: None,
                    }),
                    scheduled_options: None,
                },
                force_customer_creation: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("PUT", "/Invoice/332", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_invoice_get_attached_file_from_invoice_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

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
                return_object: None,
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
async fn test_invoice_get_invoice_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

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
async fn test_invoice_get_invoice_number_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

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
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

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
                export_format: None,
                parameters: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/Query/invoices/8cfec329267",
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
async fn test_invoice_list_invoices_org_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

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
                export_format: None,
                parameters: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/Query/invoices/org/123",
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
async fn test_invoice_send_invoice_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

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
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/Invoice/send/23548884",
        Some(HashMap::from([
            ("attachfile".to_string(), "true".to_string()),
            ("mail2".to_string(), "tamara@example.com".to_string()),
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
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

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
