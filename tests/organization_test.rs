use payabli_api::prelude::*;

mod wire_test_utils;

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_organization_add_organization_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .organization
        .add_organization(
            &AddOrganizationRequest {
                billing_info: Some(Instrument {
                    ach_account: Achaccount("123123123".to_string()),
                    ach_routing: Achrouting("123123123".to_string()),
                    billing_address: Some(BillingAddressNullable("123 Walnut Street".to_string())),
                    billing_city: Some(BillingCityNullable("Johnson City".to_string())),
                    billing_country: Some(BillingCountryNullable("US".to_string())),
                    billing_state: Some(BillingStateNullable("TN".to_string())),
                    billing_zip: Some(BillingZip("37615".to_string())),
                }),
                contacts: Some(ContactsField(Some(vec![Contacts {
                    contact_email: Some(Email("herman@hermanscoatings.com".to_string())),
                    contact_name: Some("Herman Martinez".to_string()),
                    contact_phone: Some("3055550000".to_string()),
                    contact_title: Some("Owner".to_string()),
                    additional_data: None,
                }]))),
                has_billing: Some(true),
                has_residual: Some(true),
                org_address: Some(Orgaddress("123 Walnut Street".to_string())),
                org_city: Some(Orgcity("Johnson City".to_string())),
                org_country: Some(Orgcountry("US".to_string())),
                org_entry_name: Some(Orgentryname("pilgrim-planner".to_string())),
                org_id: Some(Orgidstring("123".to_string())),
                org_logo: Some(FileContent {
                    f_content: Some("TXkgdGVzdCBmaWxlHJ==...".to_string()),
                    filename: Some("my-doc.pdf".to_string()),
                    ftype: Some(FileContentFtype::Pdf),
                    furl: Some("https://mysite.com/my-doc.pdf".to_string()),
                }),
                org_name: Orgname("Pilgrim Planner".to_string()),
                org_parent_id: Some(OrgParentId(236)),
                org_state: Some(Orgstate("TN".to_string())),
                org_timezone: Some(Orgtimezone(-5)),
                org_type: Orgtype(0),
                org_website: Some(Orgwebsite("www.pilgrimageplanner.com".to_string())),
                org_zip: Some(Orgzip("37615".to_string())),
                reply_to_email: ReplyToEmail("email@example.com".to_string()),
                services: None,
            },
            Some(
                RequestOptions::new()
                    .additional_header("idempotencyKey", "6B29FC40-CA47-1067-B31D-00DD010662DA"),
            ),
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("POST", "/Organization", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_organization_delete_organization_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client.organization.delete_organization(123, None).await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("DELETE", "/Organization/123", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_organization_edit_organization_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .organization
        .edit_organization(
            123,
            &OrganizationData {
                contacts: Some(ContactsField(Some(vec![Contacts {
                    contact_email: Some(Email("herman@hermanscoatings.com".to_string())),
                    contact_name: Some("Herman Martinez".to_string()),
                    contact_phone: Some("3055550000".to_string()),
                    contact_title: Some("Owner".to_string()),
                    additional_data: None,
                }]))),
                org_address: Some(Orgaddress("123 Walnut Street".to_string())),
                org_city: Some(Orgcity("Johnson City".to_string())),
                org_country: Some(Orgcountry("US".to_string())),
                org_entry_name: Some(Orgentryname("pilgrim-planner".to_string())),
                organization_data_org_id: Some(Orgidstring("123".to_string())),
                org_name: Some(Orgname("Pilgrim Planner".to_string())),
                org_state: Some(Orgstate("TN".to_string())),
                org_timezone: Some(Orgtimezone(-5)),
                org_type: Some(Orgtype(0)),
                org_website: Some(Orgwebsite("www.pilgrimageplanner.com".to_string())),
                org_zip: Some(Orgzip("37615".to_string())),
                services: None,
                billing_info: None,
                has_billing: None,
                has_residual: None,
                org_logo: None,
                org_parent_id: None,
                reply_to_email: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("PUT", "/Organization/123", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_organization_get_basic_organization_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .organization
        .get_basic_organization(&"8cfec329267".to_string(), None)
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("GET", "/Organization/basic/8cfec329267", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_organization_get_basic_organization_by_id_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .organization
        .get_basic_organization_by_id(123, None)
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("GET", "/Organization/basicById/123", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_organization_get_organization_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client.organization.get_organization(123, None).await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("GET", "/Organization/read/123", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_organization_get_settings_organization_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .organization
        .get_settings_organization(123, None)
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("GET", "/Organization/settings/123", None, 1)
        .await
        .unwrap();
}
