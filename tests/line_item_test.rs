use payabli_api::prelude::*;

mod wire_test_utils;

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_line_item_add_item_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .line_item
        .add_item(
            &"47cae3d74".to_string(),
            &LineItem {
                item_categories: None,
                item_commodity_code: Some(ItemCommodityCode("010".to_string())),
                item_cost: 12.45,
                item_description: Some(ItemDescription("Deposit for materials".to_string())),
                item_mode: Some(0),
                item_product_code: Some(ItemProductCode("M-DEPOSIT".to_string())),
                item_product_name: Some(ItemProductName("Materials deposit".to_string())),
                item_qty: 1,
                item_unit_of_measure: Some(ItemUnitofMeasure("SqFt".to_string())),
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("POST", "/LineItem/47cae3d74", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_line_item_delete_item_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client.line_item.delete_item(700, None).await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("DELETE", "/LineItem/700", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_line_item_get_item_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client.line_item.get_item(700, None).await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("GET", "/LineItem/700", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_line_item_list_line_items_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .line_item
        .list_line_items(
            &"8cfec329267".to_string(),
            &ListLineItemsQueryRequest {
                from_record: Some(251),
                limit_record: Some(0),
                sort_by: Some("desc(field_name)".to_string()),
                parameters: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/Query/lineitems/8cfec329267",
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
async fn test_line_item_update_item_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .line_item
        .update_item(
            700,
            &LineItem {
                item_categories: None,
                item_commodity_code: None,
                item_cost: 12.45,
                item_description: None,
                item_mode: None,
                item_product_code: None,
                item_product_name: None,
                item_qty: 1,
                item_unit_of_measure: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("PUT", "/LineItem/700", None, 1)
        .await
        .unwrap();
}
