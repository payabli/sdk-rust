use payabli_api::prelude::*;

mod wire_test_utils;

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_check_capture_check_processing_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .check_capture
        .check_processing(
            &CheckCaptureRequestBody {
                entry_point: Entry("47abcfea12".to_string()),
                front_image: "/9j/4AAQSkZJRgABAQEASABIAAD...".to_string(),
                rear_image: "/9j/4AAQSkZJRgABAQEASABIAAD...".to_string(),
                check_amount: 12550,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("POST", "/CheckCapture/CheckProcessing", None, 1)
        .await
        .unwrap();
}
