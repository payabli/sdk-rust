use payabli_api::prelude::*;

mod wire_test_utils;

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_export_export_applications_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .export
        .export_applications(
            &ExportFormat1::Csv,
            123,
            &ExportApplicationsQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                parameters: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/Export/boarding/csv/123",
        Some(HashMap::from([
            (
                "columnsExport".to_string(),
                "BatchDate:Batch_Date,PaypointName:Legal_name".to_string(),
            ),
            ("fromRecord".to_string(), "251".to_string()),
            ("limitRecord".to_string(), "1000".to_string()),
        ])),
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_export_export_batch_details_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .export
        .export_batch_details(
            &ExportFormat1::Csv,
            &"8cfec329267".to_string(),
            &ExportBatchDetailsQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                parameters: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/Export/batchDetails/csv/8cfec329267",
        Some(HashMap::from([
            (
                "columnsExport".to_string(),
                "BatchDate:Batch_Date,PaypointName:Legal_name".to_string(),
            ),
            ("fromRecord".to_string(), "251".to_string()),
            ("limitRecord".to_string(), "1000".to_string()),
        ])),
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_export_export_batch_details_org_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .export
        .export_batch_details_org(
            &ExportFormat1::Csv,
            123,
            &ExportBatchDetailsOrgQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                parameters: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/Export/batchDetails/csv/org/123",
        Some(HashMap::from([
            (
                "columnsExport".to_string(),
                "BatchDate:Batch_Date,PaypointName:Legal_name".to_string(),
            ),
            ("fromRecord".to_string(), "251".to_string()),
            ("limitRecord".to_string(), "1000".to_string()),
        ])),
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_export_export_batches_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .export
        .export_batches(
            &ExportFormat1::Csv,
            &"8cfec329267".to_string(),
            &ExportBatchesQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                parameters: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/Export/batches/csv/8cfec329267",
        Some(HashMap::from([
            (
                "columnsExport".to_string(),
                "BatchDate:Batch_Date,PaypointName:Legal_name".to_string(),
            ),
            ("fromRecord".to_string(), "251".to_string()),
            ("limitRecord".to_string(), "1000".to_string()),
        ])),
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_export_export_batches_org_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .export
        .export_batches_org(
            &ExportFormat1::Csv,
            123,
            &ExportBatchesOrgQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                parameters: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/Export/batches/csv/org/123",
        Some(HashMap::from([
            (
                "columnsExport".to_string(),
                "BatchDate:Batch_Date,PaypointName:Legal_name".to_string(),
            ),
            ("fromRecord".to_string(), "251".to_string()),
            ("limitRecord".to_string(), "1000".to_string()),
        ])),
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_export_export_batches_out_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .export
        .export_batches_out(
            &ExportFormat1::Csv,
            &"8cfec329267".to_string(),
            &ExportBatchesOutQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                parameters: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/Export/batchesOut/csv/8cfec329267",
        Some(HashMap::from([
            (
                "columnsExport".to_string(),
                "BatchDate:Batch_Date,PaypointName:Legal_name".to_string(),
            ),
            ("fromRecord".to_string(), "251".to_string()),
            ("limitRecord".to_string(), "1000".to_string()),
        ])),
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_export_export_batches_out_org_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .export
        .export_batches_out_org(
            &ExportFormat1::Csv,
            123,
            &ExportBatchesOutOrgQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                parameters: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/Export/batchesOut/csv/org/123",
        Some(HashMap::from([
            (
                "columnsExport".to_string(),
                "BatchDate:Batch_Date,PaypointName:Legal_name".to_string(),
            ),
            ("fromRecord".to_string(), "251".to_string()),
            ("limitRecord".to_string(), "1000".to_string()),
        ])),
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_export_export_bills_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .export
        .export_bills(
            &ExportFormat1::Csv,
            &"8cfec329267".to_string(),
            &ExportBillsQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                parameters: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/Export/bills/csv/8cfec329267",
        Some(HashMap::from([
            (
                "columnsExport".to_string(),
                "BatchDate:Batch_Date,PaypointName:Legal_name".to_string(),
            ),
            ("fromRecord".to_string(), "251".to_string()),
            ("limitRecord".to_string(), "1000".to_string()),
        ])),
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_export_export_bills_org_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .export
        .export_bills_org(
            &ExportFormat1::Csv,
            123,
            &ExportBillsOrgQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                parameters: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/Export/bills/csv/org/123",
        Some(HashMap::from([
            (
                "columnsExport".to_string(),
                "BatchDate:Batch_Date,PaypointName:Legal_name".to_string(),
            ),
            ("fromRecord".to_string(), "251".to_string()),
            ("limitRecord".to_string(), "1000".to_string()),
        ])),
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_export_export_chargebacks_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .export
        .export_chargebacks(
            &ExportFormat1::Csv,
            &"8cfec329267".to_string(),
            &ExportChargebacksQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                parameters: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/Export/chargebacks/csv/8cfec329267",
        Some(HashMap::from([
            (
                "columnsExport".to_string(),
                "BatchDate:Batch_Date,PaypointName:Legal_name".to_string(),
            ),
            ("fromRecord".to_string(), "251".to_string()),
            ("limitRecord".to_string(), "1000".to_string()),
        ])),
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_export_export_chargebacks_org_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .export
        .export_chargebacks_org(
            &ExportFormat1::Csv,
            123,
            &ExportChargebacksOrgQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                parameters: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/Export/chargebacks/csv/org/123",
        Some(HashMap::from([
            (
                "columnsExport".to_string(),
                "BatchDate:Batch_Date,PaypointName:Legal_name".to_string(),
            ),
            ("fromRecord".to_string(), "251".to_string()),
            ("limitRecord".to_string(), "1000".to_string()),
        ])),
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_export_export_customers_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .export
        .export_customers(
            &ExportFormat1::Csv,
            &"8cfec329267".to_string(),
            &ExportCustomersQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                parameters: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/Export/customers/csv/8cfec329267",
        Some(HashMap::from([
            (
                "columnsExport".to_string(),
                "BatchDate:Batch_Date,PaypointName:Legal_name".to_string(),
            ),
            ("fromRecord".to_string(), "251".to_string()),
            ("limitRecord".to_string(), "1000".to_string()),
        ])),
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_export_export_customers_org_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .export
        .export_customers_org(
            &ExportFormat1::Csv,
            123,
            &ExportCustomersOrgQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                parameters: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/Export/customers/csv/org/123",
        Some(HashMap::from([
            (
                "columnsExport".to_string(),
                "BatchDate:Batch_Date,PaypointName:Legal_name".to_string(),
            ),
            ("fromRecord".to_string(), "251".to_string()),
            ("limitRecord".to_string(), "1000".to_string()),
        ])),
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_export_export_invoices_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .export
        .export_invoices(
            &ExportFormat1::Csv,
            &"8cfec329267".to_string(),
            &ExportInvoicesQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                parameters: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/Export/invoices/csv/8cfec329267",
        Some(HashMap::from([
            (
                "columnsExport".to_string(),
                "BatchDate:Batch_Date,PaypointName:Legal_name".to_string(),
            ),
            ("fromRecord".to_string(), "251".to_string()),
            ("limitRecord".to_string(), "1000".to_string()),
        ])),
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_export_export_invoices_org_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .export
        .export_invoices_org(
            &ExportFormat1::Csv,
            123,
            &ExportInvoicesOrgQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                parameters: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/Export/invoices/csv/org/123",
        Some(HashMap::from([
            (
                "columnsExport".to_string(),
                "BatchDate:Batch_Date,PaypointName:Legal_name".to_string(),
            ),
            ("fromRecord".to_string(), "251".to_string()),
            ("limitRecord".to_string(), "1000".to_string()),
        ])),
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_export_export_organizations_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .export
        .export_organizations(
            &ExportFormat1::Csv,
            123,
            &ExportOrganizationsQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                parameters: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/Export/organizations/csv/org/123",
        Some(HashMap::from([
            (
                "columnsExport".to_string(),
                "BatchDate:Batch_Date,PaypointName:Legal_name".to_string(),
            ),
            ("fromRecord".to_string(), "251".to_string()),
            ("limitRecord".to_string(), "1000".to_string()),
        ])),
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_export_export_payout_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .export
        .export_payout(
            &ExportFormat1::Csv,
            &"8cfec329267".to_string(),
            &ExportPayoutQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                parameters: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/Export/payouts/csv/8cfec329267",
        Some(HashMap::from([
            (
                "columnsExport".to_string(),
                "BatchDate:Batch_Date,PaypointName:Legal_name".to_string(),
            ),
            ("fromRecord".to_string(), "251".to_string()),
            ("limitRecord".to_string(), "1000".to_string()),
        ])),
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_export_export_payout_org_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .export
        .export_payout_org(
            &ExportFormat1::Csv,
            123,
            &ExportPayoutOrgQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                parameters: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/Export/payouts/csv/org/123",
        Some(HashMap::from([
            (
                "columnsExport".to_string(),
                "BatchDate:Batch_Date,PaypointName:Legal_name".to_string(),
            ),
            ("fromRecord".to_string(), "251".to_string()),
            ("limitRecord".to_string(), "1000".to_string()),
        ])),
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_export_export_paypoints_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .export
        .export_paypoints(
            &ExportFormat1::Csv,
            123,
            &ExportPaypointsQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                parameters: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/Export/paypoints/csv/123",
        Some(HashMap::from([
            (
                "columnsExport".to_string(),
                "BatchDate:Batch_Date,PaypointName:Legal_name".to_string(),
            ),
            ("fromRecord".to_string(), "251".to_string()),
            ("limitRecord".to_string(), "1000".to_string()),
        ])),
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_export_export_settlements_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .export
        .export_settlements(
            &ExportFormat1::Csv,
            &"8cfec329267".to_string(),
            &ExportSettlementsQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                parameters: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/Export/settlements/csv/8cfec329267",
        Some(HashMap::from([
            (
                "columnsExport".to_string(),
                "BatchDate:Batch_Date,PaypointName:Legal_name".to_string(),
            ),
            ("fromRecord".to_string(), "251".to_string()),
            ("limitRecord".to_string(), "1000".to_string()),
        ])),
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_export_export_settlements_org_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .export
        .export_settlements_org(
            &ExportFormat1::Csv,
            123,
            &ExportSettlementsOrgQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                parameters: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/Export/settlements/csv/org/123",
        Some(HashMap::from([
            (
                "columnsExport".to_string(),
                "BatchDate:Batch_Date,PaypointName:Legal_name".to_string(),
            ),
            ("fromRecord".to_string(), "251".to_string()),
            ("limitRecord".to_string(), "1000".to_string()),
        ])),
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_export_export_subscriptions_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .export
        .export_subscriptions(
            &ExportFormat1::Csv,
            &"8cfec329267".to_string(),
            &ExportSubscriptionsQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                parameters: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/Export/subscriptions/csv/8cfec329267",
        Some(HashMap::from([
            (
                "columnsExport".to_string(),
                "BatchDate:Batch_Date,PaypointName:Legal_name".to_string(),
            ),
            ("fromRecord".to_string(), "251".to_string()),
            ("limitRecord".to_string(), "1000".to_string()),
        ])),
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_export_export_subscriptions_org_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .export
        .export_subscriptions_org(
            &ExportFormat1::Csv,
            123,
            &ExportSubscriptionsOrgQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                parameters: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/Export/subscriptions/csv/org/123",
        Some(HashMap::from([
            (
                "columnsExport".to_string(),
                "BatchDate:Batch_Date,PaypointName:Legal_name".to_string(),
            ),
            ("fromRecord".to_string(), "251".to_string()),
            ("limitRecord".to_string(), "1000".to_string()),
        ])),
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_export_export_transactions_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .export
        .export_transactions(
            &ExportFormat1::Csv,
            &"8cfec329267".to_string(),
            &ExportTransactionsQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                parameters: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/Export/transactions/csv/8cfec329267",
        Some(HashMap::from([
            (
                "columnsExport".to_string(),
                "BatchDate:Batch_Date,PaypointName:Legal_name".to_string(),
            ),
            ("fromRecord".to_string(), "251".to_string()),
            ("limitRecord".to_string(), "1000".to_string()),
        ])),
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_export_export_transactions_org_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .export
        .export_transactions_org(
            &ExportFormat1::Csv,
            123,
            &ExportTransactionsOrgQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                parameters: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/Export/transactions/csv/org/123",
        Some(HashMap::from([
            (
                "columnsExport".to_string(),
                "BatchDate:Batch_Date,PaypointName:Legal_name".to_string(),
            ),
            ("fromRecord".to_string(), "251".to_string()),
            ("limitRecord".to_string(), "1000".to_string()),
        ])),
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_export_export_transfer_details_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .export
        .export_transfer_details(
            &ExportFormat1::Csv,
            &"8cfec329267".to_string(),
            1000000,
            &ExportTransferDetailsQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                sort_by: Some("desc(field_name)".to_string()),
                parameters: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/Export/transferDetails/csv/8cfec329267/1000000",
        Some(HashMap::from([
            (
                "columnsExport".to_string(),
                "BatchDate:Batch_Date,PaypointName:Legal_name".to_string(),
            ),
            ("fromRecord".to_string(), "251".to_string()),
            ("limitRecord".to_string(), "1000".to_string()),
            ("sortBy".to_string(), "desc(field_name)".to_string()),
        ])),
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_export_export_transfers_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .export
        .export_transfers(
            &"8cfec329267".to_string(),
            &ExportTransfersQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                sort_by: Some("desc(field_name)".to_string()),
                parameters: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/Export/transfers/8cfec329267",
        Some(HashMap::from([
            (
                "columnsExport".to_string(),
                "BatchDate:Batch_Date,PaypointName:Legal_name".to_string(),
            ),
            ("fromRecord".to_string(), "251".to_string()),
            ("limitRecord".to_string(), "1000".to_string()),
            ("sortBy".to_string(), "desc(field_name)".to_string()),
        ])),
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_export_export_vendors_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .export
        .export_vendors(
            &ExportFormat1::Csv,
            &"8cfec329267".to_string(),
            &ExportVendorsQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                parameters: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/Export/vendors/csv/8cfec329267",
        Some(HashMap::from([
            (
                "columnsExport".to_string(),
                "BatchDate:Batch_Date,PaypointName:Legal_name".to_string(),
            ),
            ("fromRecord".to_string(), "251".to_string()),
            ("limitRecord".to_string(), "1000".to_string()),
        ])),
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_export_export_vendors_org_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .export
        .export_vendors_org(
            &ExportFormat1::Csv,
            123,
            &ExportVendorsOrgQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                parameters: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/Export/vendors/csv/org/123",
        Some(HashMap::from([
            (
                "columnsExport".to_string(),
                "BatchDate:Batch_Date,PaypointName:Legal_name".to_string(),
            ),
            ("fromRecord".to_string(), "251".to_string()),
            ("limitRecord".to_string(), "1000".to_string()),
        ])),
        1,
    )
    .await
    .unwrap();
}
