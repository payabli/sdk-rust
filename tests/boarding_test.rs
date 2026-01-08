use payabli_api::prelude::*;

mod wire_test_utils;

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_boarding_add_application_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client.boarding.add_application(&AddApplicationRequest::ApplicationDataPayIn(ApplicationDataPayIn {
        services: ApplicationDataPayInServices {
        ach: ApplicationDataPayInServicesAch {
        ach_setup_fields: AchSetup {
        accept_ccd: None,
        accept_ppd: None,
        accept_web: None
        }
        },
        card: ApplicationDataPayInServicesCard {
        card_setup_fields: CardSetup {
        accept_amex: Some(true),
        accept_discover: Some(true),
        accept_mastercard: Some(true),
        accept_visa: Some(true)
        }
        },
        odp: None
        },
        annual_revenue: Some(Annualrevenue(Some(1000.0))),
        average_bill_size: Some(BoardingAverageBillSize("500".to_string())),
        average_monthly_bill: Some(BoardingAvgMonthlyBill("5650".to_string())),
        avgmonthly: Some(Avgmonthly(Some(1000.0))),
        baddress: Some(Baddress1("123 Walnut Street".to_string())),
        baddress_1: Some(Baddress2("Suite 103".to_string())),
        bank_data: ApplicationDataPayInBankData {
        bank_fields: Bank {
        id: None,
        account_id: None,
        nickname: None,
        bank_name: None,
        routing_account: None,
        account_number: None,
        type_account: None,
        bank_account_holder_name: None,
        bank_account_holder_type: None,
        bank_account_function: None,
        verified: None,
        status: None,
        services: None
        }
        },
        bcity: Some(Bcity("New Vegas".to_string())),
        bcountry: Some(Bcountry("US".to_string())),
        binperson: Some(Binperson(60)),
        binphone: Some(Binphone(20)),
        binweb: Some(Binweb(20)),
        boarding_link_id: None,
        bstate: Some(Bstate("FL".to_string())),
        bsummary: Some(Bsummary("Brick and mortar store that sells office supplies".to_string())),
        btype: Some(OwnType::LimitedLiabilityCompany),
        bzip: Some(Bzip("33000".to_string())),
        contacts: Some(vec![ApplicationDataPayInContactsItem {
        contacts_fields: Contacts {
        contact_email: Some(Email("herman@hermanscoatings.com".to_string())),
        contact_name: Some("Herman Martinez".to_string()),
        contact_phone: Some("3055550000".to_string()),
        contact_title: Some("Owner".to_string()),
        additional_data: None
        }
        }]),
        credit_limit: Some("creditLimit".to_string()),
        dba_name: Some(Dbaname("Sunshine Gutters".to_string())),
        ein: Some(Ein("123456789".to_string())),
        externalpaypoint_id: None,
        faxnumber: Some(FaxNumber("1234567890".to_string())),
        highticketamt: Some(Highticketamt(Some(1000.0))),
        legal_name: Some(Legalname("Sunshine Services, LLC".to_string())),
        license: Some(License("2222222FFG".to_string())),
        licstate: Some(Licensestate("CA".to_string())),
        maddress: Some(Maddress("123 Walnut Street".to_string())),
        maddress_1: Some(Maddress1("STE 900".to_string())),
        mcc: Some(Mcc("7777".to_string())),
        mcity: Some(Mcity("Johnson City".to_string())),
        mcountry: Some(Mcountry("US".to_string())),
        mstate: Some(Mstate("TN".to_string())),
        mzip: Some(Mzip("37615".to_string())),
        org_id: Some(Orgid(123)),
        ownership: Some(vec![ApplicationDataPayInOwnershipItem {
        owners_fields: Owners {
        ownername: Some("John Smith".to_string()),
        ownertitle: Some("CEO".to_string()),
        ownerpercent: Some(100),
        ownerssn: Some("123456789".to_string()),
        ownerdob: Some("01/01/1990".to_string()),
        ownerphone_1: Some("555888111".to_string()),
        ownerphone_2: Some("555888111".to_string()),
        owneremail: Some(Email("test@email.com".to_string())),
        ownerdriver: Some("CA6677778".to_string()),
        oaddress: Some("33 North St".to_string()),
        ocity: Some("Any City".to_string()),
        ocountry: Some("US".to_string()),
        odriverstate: Some("CA".to_string()),
        ostate: Some("CA".to_string()),
        ozip: Some("55555".to_string()),
        additional_data: None
        }
        }]),
        phonenumber: PhoneNumber("1234567890".to_string()),
        processing_region: "US".to_string(),
        recipient_email: Some(Email("josephray@example.com".to_string())),
        recipient_email_notification: Some(RecipientEmailNotification(true)),
        resumable: Some(Resumable(true)),
        signer: SignerDataRequest {
        name: Some(SignerName("John Smith".to_string())),
        ssn: Some(SignerSsn("123456789".to_string())),
        dob: Some(SignerDob("01/01/1976".to_string())),
        phone: Some(SignerPhone("555888111".to_string())),
        email: Some(Email("test@email.com".to_string())),
        address: Some(Signeraddress("33 North St".to_string())),
        address_1: Some(SignerAddress1("STE 900".to_string())),
        city: Some(SignerCity("Bristol".to_string())),
        country: Some(SignerCountry("US".to_string())),
        state: Some(SignerState("TN".to_string())),
        zip: Some(SignerZip("55555".to_string())),
        acceptance: None,
        signed_document_reference: Some(SignedDocumentReference("https://example.com/signed-document.pdf".to_string())),
        pci_attestation: Some(PciAttestation(Some(true))),
        attestation_date: Some(AttestationDate("04/20/2025".to_string())),
        additional_data: Some(AdditionalDataString("{\"deviceId\":\"499585-389fj484-3jcj8hj3\",\"session\":\"fifji4-fiu443-fn4843\",\"timeWithCompany\":\"6 Years\"}".to_string())),
        sign_date: Some(SignDate("04/20/2025".to_string()))
        },
        startdate: Some(Busstartdate("01/01/1990".to_string())),
        tax_fill_name: Some(Taxfillname("Sunshine LLC".to_string())),
        template_id: Some(TemplateId(22)),
        ticketamt: Some(Ticketamt(Some(1000.0))),
        website: Some(Website("www.example.com".to_string())),
        when_charged: Whencharged::WhenServiceProvided,
        when_delivered: Whendelivered::Over30Days,
        when_provided: Whenprovided::ThirtyDaysOrLess,
        when_refunded: Whenrefunded::ThirtyDaysOrLess,
        additional_data: None,
        rep_code: None,
        rep_name: None,
        rep_office: None,
        on_create: None
        }), None).await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("POST", "/Boarding/app", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_boarding_delete_application_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client.boarding.delete_application(352, None).await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("DELETE", "/Boarding/app/352", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_boarding_get_application_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client.boarding.get_application(352, None).await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("GET", "/Boarding/read/352", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_boarding_get_application_by_auth_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .boarding
        .get_application_by_auth(
            &"17E".to_string(),
            &RequestAppByAuth {
                email: Some(Email("admin@email.com".to_string())),
                reference_id: Some("n6UCd1f1ygG7".to_string()),
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("POST", "/Boarding/read/17E", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_boarding_get_by_id_link_application_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client.boarding.get_by_id_link_application(91, None).await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("GET", "/Boarding/linkbyId/91", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_boarding_get_by_template_id_link_application_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .boarding
        .get_by_template_id_link_application(80.0, None)
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("GET", "/Boarding/linkbyTemplate/80", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_boarding_get_external_application_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .boarding
        .get_external_application(
            352,
            &"mail2".to_string(),
            &GetExternalApplicationQueryRequest { send_email: None },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("PUT", "/Boarding/applink/352/mail2", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_boarding_get_link_application_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .boarding
        .get_link_application(&"myorgaccountname-00091".to_string(), None)
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("GET", "/Boarding/link/myorgaccountname-00091", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_boarding_list_applications_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .boarding
        .list_applications(
            123,
            &ListApplicationsQueryRequest {
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
        "/Query/boarding/123",
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
async fn test_boarding_list_boarding_links_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .boarding
        .list_boarding_links(
            123,
            &ListBoardingLinksQueryRequest {
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
        "/Query/boardinglinks/123",
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
async fn test_boarding_update_application_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .boarding
        .update_application(
            352,
            &ApplicationData {
                services: None,
                annual_revenue: None,
                attachments: None,
                avgmonthly: None,
                baddress: None,
                baddress_1: None,
                bank_data: None,
                bcity: None,
                bcountry: None,
                binperson: None,
                binphone: None,
                binweb: None,
                bstate: None,
                bsummary: None,
                btype: None,
                bzip: None,
                contacts: None,
                dbaname: None,
                ein: None,
                external_paypoint_id: None,
                faxnumber: None,
                highticketamt: None,
                legalname: None,
                license: None,
                licstate: None,
                maddress: None,
                maddress_1: None,
                mcc: None,
                mcity: None,
                mcountry: None,
                mstate: None,
                mzip: None,
                org_id: None,
                ownership: None,
                payout_average_monthly_volume: None,
                payout_average_ticket_limit: None,
                payout_credit_limit: None,
                payout_high_ticket_amount: None,
                phonenumber: None,
                recipient_email: None,
                recipient_email_notification: None,
                resumable: None,
                signer: None,
                startdate: None,
                taxfillname: None,
                template_id: None,
                ticketamt: None,
                website: None,
                when_charged: None,
                when_delivered: None,
                when_provided: None,
                when_refunded: None,
                rep_code: None,
                rep_name: None,
                rep_office: None,
                on_create: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("PUT", "/Boarding/app/352", None, 1)
        .await
        .unwrap();
}
