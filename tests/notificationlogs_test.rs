use payabli_api::prelude::*;

mod wire_test_utils;

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_notificationlogs_search_notification_logs_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .notificationlogs
        .search_notification_logs(
            &SearchNotificationLogsRequest {
                page_size: Some(Pagesize(20)),
                body: NotificationLogSearchRequest {
                    start_date: DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z").unwrap(),
                    end_date: DateTime::parse_from_rfc3339("2024-01-31T23:59:59Z").unwrap(),
                    notification_event: Some("ActivatedMerchant".to_string()),
                    succeeded: Some(true),
                    org_id: Some(12345),
                    paypoint_id: None,
                },
                page: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "POST",
        "/v2/notificationlogs",
        Some(HashMap::from([("PageSize".to_string(), "20".to_string())])),
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_notificationlogs_get_notification_log_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .notificationlogs
        .get_notification_log(
            &Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap(),
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/v2/notificationlogs/550e8400-e29b-41d4-a716-446655440000",
        None,
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_notificationlogs_retry_notification_log_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .notificationlogs
        .retry_notification_log(
            &Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap(),
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/v2/notificationlogs/550e8400-e29b-41d4-a716-446655440000/retry",
        None,
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_notificationlogs_bulk_retry_notification_logs_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .notificationlogs
        .bulk_retry_notification_logs(
            &BulkRetryRequest(vec![
                Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap(),
                Uuid::parse_str("550e8400-e29b-41d4-a716-446655440001").unwrap(),
                Uuid::parse_str("550e8400-e29b-41d4-a716-446655440002").unwrap(),
            ]),
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("POST", "/v2/notificationlogs/retry", None, 1)
        .await
        .unwrap();
}
