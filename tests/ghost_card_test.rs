use payabli_api::prelude::*;

mod wire_test_utils;

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_ghost_card_create_ghost_card_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .ghost_card
        .create_ghost_card(
            &Entry("8cfec2e0fa".to_string()),
            &CreateGhostCardRequestBody {
                vendor_id: 42,
                expense_limit: 500.0,
                amount: 500.0,
                max_number_of_uses: 3,
                exact_amount: false,
                expense_limit_period: "monthly".to_string(),
                billing_cycle: "monthly".to_string(),
                billing_cycle_day: "1".to_string(),
                daily_transaction_count: 5,
                daily_amount_limit: 200.0,
                transaction_amount_limit: 100,
                mcc: Some("5411".to_string()),
                tcc: Some("R".to_string()),
                misc_1: Some("PO-98765".to_string()),
                misc_2: Some("Dept-Finance".to_string()),
                expiration_date: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("POST", "/MoneyOutCard/GhostCard/8cfec2e0fa", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_ghost_card_update_card_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .ghost_card
        .update_card(
            &Entry("8cfec2e0fa".to_string()),
            &UpdateCardRequestBody {
                card_token: "gc_abc123def456".to_string(),
                status: Some(CardStatus::Cancelled),
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("PATCH", "/MoneyOutCard/card/8cfec2e0fa", None, 1)
        .await
        .unwrap();
}
