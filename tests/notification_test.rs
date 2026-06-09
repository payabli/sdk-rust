use payabli_api::prelude::*;

mod wire_test_utils;

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_notification_add_notification_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .notification
        .add_notification(
            &AddNotificationRequest::NotificationStandardRequest(NotificationStandardRequest {
                content: Some(NotificationStandardRequestContent {
                    event_type: Some(
                        NotificationStandardRequestContentEventType::CreatedApplication,
                    ),
                    ..Default::default()
                }),
                frequency: NotificationStandardRequestFrequency::Untilcancelled,
                method: NotificationStandardRequestMethod::Web,
                owner_id: Some(Ownerid(236)),
                owner_type: Ownertype(0),
                status: Some(Statusnotification(1)),
                target: "https://webhook.site/2871b8f8-edc7-441a-b376-98d8c8e33275".to_string(),
            }),
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("POST", "/Notification", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_notification_get_notification_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .notification
        .get_notification(&"1717".to_string(), None)
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("GET", "/Notification/1717", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_notification_update_notification_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .notification
        .update_notification(
            &"1717".to_string(),
            &UpdateNotificationRequest::NotificationStandardRequest(NotificationStandardRequest {
                content: Some(NotificationStandardRequestContent {
                    event_type: Some(NotificationStandardRequestContentEventType::ApprovedPayment),
                    ..Default::default()
                }),
                frequency: NotificationStandardRequestFrequency::Untilcancelled,
                method: NotificationStandardRequestMethod::Email,
                owner_id: Some(Ownerid(136)),
                owner_type: Ownertype(0),
                status: Some(Statusnotification(1)),
                target: "newemail@email.com".to_string(),
            }),
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("PUT", "/Notification/1717", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_notification_delete_notification_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .notification
        .delete_notification(&"1717".to_string(), None)
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("DELETE", "/Notification/1717", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_notification_get_report_file_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client.notification.get_report_file(1000000, None).await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("GET", "/Export/notificationReport/1000000", None, 1)
        .await
        .unwrap();
}
