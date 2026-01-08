use payabli_api::prelude::*;

mod wire_test_utils;

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_payment_link_add_pay_link_from_invoice_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client.payment_link.add_pay_link_from_invoice(23548884, &AddPayLinkFromInvoiceRequest {
        mail_2: Some("jo@example.com; ceo@example.com".to_string()),
        body: PaymentPageRequestBody {
        contact_us: Some(ContactElement {
        email_label: Some("Email".to_string()),
        enabled: Some(Enabled(true)),
        header: Some("Contact Us".to_string()),
        order: Some(Order(0)),
        payment_icons: Some(true),
        phone_label: Some("Phone".to_string())
        }),
        invoices: Some(InvoiceElement {
        enabled: Some(Enabled(true)),
        invoice_link: Some(LabelElement {
        enabled: Some(Enabled(true)),
        label: Some("View Invoice".to_string()),
        order: Some(Order(0))
        }),
        order: Some(Order(0)),
        view_invoice_details: Some(LabelElement {
        enabled: Some(Enabled(true)),
        label: Some("Invoice Details".to_string()),
        order: Some(Order(0))
        })
        }),
        logo: Some(Element {
        enabled: Some(Enabled(true)),
        order: Some(Order(0))
        }),
        message_before_paying: Some(LabelElement {
        enabled: Some(Enabled(true)),
        label: Some("Please review your payment details".to_string()),
        order: Some(Order(0))
        }),
        notes: Some(NoteElement {
        enabled: Some(Enabled(true)),
        header: Some("Additional Notes".to_string()),
        order: Some(Order(0)),
        placeholder: Some("Enter any additional notes here".to_string()),
        value: Some("".to_string())
        }),
        page: Some(PageElement {
        description: Some("Complete your payment securely".to_string()),
        enabled: Some(Enabled(true)),
        header: Some("Payment Page".to_string()),
        order: Some(Order(0))
        }),
        payment_button: Some(LabelElement {
        enabled: Some(Enabled(true)),
        label: Some("Pay Now".to_string()),
        order: Some(Order(0))
        }),
        payment_methods: Some(MethodElement {
        all_methods_checked: Some(true),
        enabled: Some(Enabled(true)),
        header: Some("Payment Methods".to_string()),
        methods: Some(MethodsList {
        amex: Some(true),
        apple_pay: Some(true),
        google_pay: None,
        discover: Some(true),
        e_check: Some(true),
        mastercard: Some(true),
        visa: Some(true)
        }),
        order: Some(Order(0)),
        settings: Some(MethodElementSettings {
        apple_pay: Some(MethodElementSettingsApplePay {
        button_style: Some(MethodElementSettingsApplePayButtonStyle::Black),
        button_type: Some(MethodElementSettingsApplePayButtonType::Pay),
        language: Some(MethodElementSettingsApplePayLanguage::EnUs)
        })
        })
        }),
        payor: Some(PayorElement {
        enabled: Some(Enabled(true)),
        fields: Some(vec![PayorFields {
        display: Some(true),
        fixed: Some(true),
        identifier: Some(true),
        label: Some("Full Name".to_string()),
        name: Some("fullName".to_string()),
        order: Some(Order(0)),
        required: Some(true),
        validation: Some("alpha".to_string()),
        value: Some("".to_string()),
        width: Some(0)
        }]),
        header: Some("Payor Information".to_string()),
        order: Some(Order(0))
        }),
        review: Some(HeaderElement {
        enabled: Some(Enabled(true)),
        header: Some("Review Payment".to_string()),
        order: Some(Order(0))
        }),
        settings: Some(PagelinkSetting {
        color: Some("#000000".to_string()),
        custom_css_url: Some("https://example.com/custom.css".to_string()),
        language: Some("en".to_string()),
        page_logo: Some(FileContent {
        f_content: Some("PHN2ZyB2aWV3Qm94PSIwIDAgODAwIDEwMDAiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyI+CiAgPCEtLSBCYWNrZ3JvdW5kIC0tPgogIDxyZWN0IHdpZHRoPSI4MDAiIGhlaWdodD0iMTAwMCIgZmlsbD0id2hpdGUiLz4KICAKICA8IS0tIENvbXBhbnkgSGVhZGVyIC0tPgogIDx0ZXh0IHg9IjQwIiB5PSI2MCIgZm9udC1mYW1pbHk9IkFyaWFsIiBmb250LXNpemU9IjI0IiBmb250LXdlaWdodD0iYm9sZCIgZmlsbD0iIzJjM2U1MCI+R3J1enlhIEFkdmVudHVyZSBPdXRmaXR0ZXJzPC90ZXh0PgogIDxsaW5lIHgxPSI0MCIgeTE9IjgwIiB4Mj0iNzYwIiB5Mj0iODAiIHN0cm9rZT0iIzJjM2U1MCIgc3Ryb2tlLXdpZHRoPSIyIi8+CiAgCiAgPCEtLSBDb21wYW55IERldGFpbHMgLS0+CiAgPHRleHQgeD0iNDAiIHk9IjExMCIgZm9udC1mYW1pbHk9IkFyaWFsIiBmb250LXNpemU9IjE0IiBmaWxsPSIjMzQ0OTVlIj4xMjMgTW91bnRhaW4gVmlldyBSb2FkPC90ZXh0PgogIDx0ZXh0IHg9IjQwIiB5PSIxMzAiIGZvbnQtZmFtaWx5PSJBcmlhbCIgZm9udC1zaXplPSIxNCIgZmlsbD0iIzM0NDk1ZSI+VGJpbGlzaSwgR2VvcmdpYSAwMTA1PC90ZXh0PgogIDx0ZXh0IHg9IjQwIiB5PSIxNTAiIGZvbnQtZmFtaWx5PSJBcmlhbCIgZm9udC1zaXplPSIxNCIgZmlsbD0iIzM0NDk1ZSI+VGVsOiArOTk1IDMyIDEyMyA0NTY3PC90ZXh0PgogIDx0ZXh0IHg9IjQwIiB5PSIxNzAiIGZvbnQtZmFtaWx5PSJBcmlhbCIgZm9udC1zaXplPSIxNCIgZmlsbD0iIzM0NDk1ZSI+RW1haWw6IGluZm9AZ3J1enlhYWR2ZW50dXJlcy5jb208L3RleHQ+CgogIDwhLS0gSW52b2ljZSBUaXRsZSAtLT4KICA8dGV4dCB4PSI2MDAiIHk9IjExMCIgZm9udC1mYW1pbHk9IkFyaWFsIiBmb250LXNpemU9IjI0IiBmb250LXdlaWdodD0iYm9sZCIgZmlsbD0iIzJjM2U1MCI+SU5WT0lDRTwvdGV4dD4KICA8dGV4dCB4PSI2MDAiIHk9IjE0MCIgZm9udC1mYW1pbHk9IkFyaWFsIiBmb250LXNpemU9IjE0IiBmaWxsPSIjMzQ0OTVlIj5EYXRlOiAxMi8xMS8yMDI0PC90ZXh0PgogIDx0ZXh0IHg9IjYwMCIgeT0iMTYwIiBmb250LWZhbWlseT0iQXJpYWwiIGZvbnQtc2l6ZT0iMTQiIGZpbGw9IiMzNDQ5NWUiPkludm9pY2UgIzogR1JaLTIwMjQtMTEyMzwvdGV4dD4KCiAgPCEtLSBCaWxsIFRvIFNlY3Rpb24gLS0+CiAgPHRleHQgeD0iNDAiIHk9IjIyMCIgZm9udC1mYW1pbHk9IkFyaWFsIiBmb250LXNpemU9IjE2IiBmb250LXdlaWdodD0iYm9sZCIgZmlsbD0iIzJjM2U1MCI+QklMTCBUTzo8L3RleHQ+CiAgPHJlY3QgeD0iNDAiIHk9IjIzNSIgd2lkdGg9IjMwMCIgaGVpZ2h0PSI4MCIgZmlsbD0iI2Y3ZjlmYSIvPgogIDx0ZXh0IHg9IjUwIiB5PSIyNjAiIGZvbnQtZmFtaWx5PSJBcmlhbCIgZm9udC1zaXplPSIxNCIgZmlsbD0iIzM0NDk1ZSI+W0N1c3RvbWVyIE5hbWVdPC90ZXh0PgogIDx0ZXh0IHg9IjUwIiB5PSIyODAiIGZvbnQtZmFtaWx5PSJBcmlhbCIgZm9udC1zaXplPSIxNCIgZmlsbD0iIzM0NDk1ZSI+W0FkZHJlc3MgTGluZSAxXTwvdGV4dD4KICA8dGV4dCB4PSI1MCIgeT0iMzAwIiBmb250LWZhbWlseT0iQXJpYWwiIGZvbnQtc2l6ZT0iMTQiIGZpbGw9IiMzNDQ5NWUiPltDaXR5LCBDb3VudHJ5XTwvdGV4dD4KCiAgPCEtLSBUYWJsZSBIZWFkZXJzIC0tPgogIDxyZWN0IHg9IjQwIiB5PSIzNDAiIHdpZHRoPSI3MjAiIGhlaWdodD0iMzAiIGZpbGw9IiMyYzNlNTAiLz4KICA8dGV4dCB4PSI1MCIgeT0iMzYwIiBmb250LWZhbWlseT0iQXJpYWwiIGZvbnQtc2l6ZT0iMTQiIGZvbnQtd2VpZ2h0PSJib2xkIiBmaWxsPSJ3aGl0ZSI+RGVzY3JpcHRpb248L3RleHQ+CiAgPHRleHQgeD0iNDUwIiB5PSIzNjAiIGZvbnQtZmFtaWx5PSJBcmlhbCIgZm9udC1zaXplPSIxNCIgZm9udC13ZWlnaHQ9ImJvbGQiIGZpbGw9IndoaXRlIj5RdWFudGl0eTwvdGV4dD4KICA8dGV4dCB4PSI1NTAiIHk9IjM2MCIgZm9udC1mYW1pbHk9IkFyaWFsIiBmb250LXNpemU9IjE0IiBmb250LXdlaWdodD0iYm9sZCIgZmlsbD0id2hpdGUiPlJhdGU8L3RleHQ+CiAgPHRleHQgeD0iNjgwIiB5PSIzNjAiIGZvbnQtZmFtaWx5PSJBcmlhbCIgZm9udC1zaXplPSIxNCIgZm9udC13ZWlnaHQ9ImJvbGQiIGZpbGw9IndoaXRlIj5BbW91bnQ8L3RleHQ+CgogIDwhLS0gVGFibGUgUm93cyAtLT4KICA8cmVjdCB4PSI0MCIgeT0iMzcwIiB3aWR0aD0iNzIwIiBoZWlnaHQ9IjMwIiBmaWxsPSIjZjdmOWZhIi8+CiAgPHRleHQgeD0iNTAiIHk9IjM5MCIgZm9udC1mYW1pbHk9IkFyaWFsIiBmb250LXNpemU9IjE0IiBmaWxsPSIjMzQ0OTVlIj5Nb3VudGFpbiBDbGltYmluZyBFcXVpcG1lbnQgUmVudGFsPC90ZXh0PgogIDx0ZXh0IHg9IjQ1MCIgeT0iMzkwIiBmb250LWZhbWlseT0iQXJpYWwiIGZvbnQtc2l6ZT0iMTQiIGZpbGw9IiMzNDQ5NWUiPjE8L3RleHQ+CiAgPHRleHQgeD0iNTUwIiB5PSIzOTAiIGZvbnQtZmFtaWx5PSJBcmlhbCIgZm9udC1zaXplPSIxNCIgZmlsbD0iIzM0NDk1ZSI+JDI1MC4wMDwvdGV4dD4KICA8dGV4dCB4PSI2ODAiIHk9IjM5MCIgZm9udC1mYW1pbHk9IkFyaWFsIiBmb250LXNpemU9IjE0IiBmaWxsPSIjMzQ0OTVlIj4kMjUwLjAwPC90ZXh0PgoKICA8cmVjdCB4PSI0MCIgeT0iNDAwIiB3aWR0aD0iNzIwIiBoZWlnaHQ9IjMwIiBmaWxsPSJ3aGl0ZSIvPgogIDx0ZXh0IHg9IjUwIiB5PSI0MjAiIGZvbnQtZmFtaWx5PSJBcmlhbCIgZm9udC1zaXplPSIxNCIgZmlsbD0iIzM0NDk1ZSI+R3VpZGVkIFRyZWsgUGFja2FnZSAtIDIgRGF5czwvdGV4dD4KICA8dGV4dCB4PSI0NTAiIHk9IjQyMCIgZm9udC1mYW1pbHk9IkFyaWFsIiBmb250LXNpemU9IjE0IiBmaWxsPSIjMzQ0OTVlIj4xPC90ZXh0PgogIDx0ZXh0IHg9IjU1MCIgeT0iNDIwIiBmb250LWZhbWlseT0iQXJpYWwiIGZvbnQtc2l6ZT0iMTQiIGZpbGw9IiMzNDQ5NWUiPiQ0MDAuMDA8L3RleHQ+CiAgPHRleHQgeD0iNjgwIiB5PSI0MjAiIGZvbnQtZmFtaWx5PSJBcmlhbCIgZm9udC1zaXplPSIxNCIgZmlsbD0iIzM0NDk1ZSI+JDQwMC4wMDwvdGV4dD4KCiAgPHJlY3QgeD0iNDAiIHk9IjQzMCIgd2lkdGg9IjcyMCIgaGVpZ2h0PSIzMCIgZmlsbD0iI2Y3ZjlmYSIvPgogIDx0ZXh0IHg9IjUwIiB5PSI0NTAiIGZvbnQtZmFtaWx5PSJBcmlhbCIgZm9udC1zaXplPSIxNCIgZmlsbD0iIzM0NDk1ZSI+U2FmZXR5IEVxdWlwbWVudCBQYWNrYWdlPC90ZXh0PgogIDx0ZXh0IHg9IjQ1MCIgeT0iNDUwIiBmb250LWZhbWlseT0iQXJpYWwiIGZvbnQtc2l6ZT0iMTQiIGZpbGw9IiMzNDQ5NWUiPjE8L3RleHQ+CiAgPHRleHQgeD0iNTUwIiB5PSI0NTAiIGZvbnQtZmFtaWx5PSJBcmlhbCIgZm9udC1zaXplPSIxNCIgZmlsbD0iIzM0NDk1ZSI+JDE1MC4wMDwvdGV4dD4KICA8dGV4dCB4PSI2ODAiIHk9IjQ1MCIgZm9udC1mYW1pbHk9IkFyaWFsIiBmb250LXNpemU9IjE0IiBmaWxsPSIjMzQ0OTVlIj4kMTUwLjAwPC90ZXh0PgoKICA8IS0tIFRvdGFscyAtLT4KICA8bGluZSB4MT0iNDAiIHkxPSI0ODAiIHgyPSI3NjAiIHkyPSI0ODAiIHN0cm9rZT0iIzJjM2U1MCIgc3Ryb2tlLXdpZHRoPSIxIi8+CiAgPHRleHQgeD0iNTUwIiB5PSI1MTAiIGZvbnQtZmFtaWx5PSJBcmlhbCIgZm9udC1zaXplPSIxNCIgZm9udC13ZWlnaHQ9ImJvbGQiIGZpbGw9IiMzNDQ5NWUiPlN1YnRvdGFsOjwvdGV4dD4KICA8dGV4dCB4PSI2ODAiIHk9IjUxMCIgZm9udC1mYW1pbHk9IkFyaWFsIiBmb250LXNpemU9IjE0IiBmaWxsPSIjMzQ0OTVlIj4kODAwLjAwPC90ZXh0PgogIDx0ZXh0IHg9IjU1MCIgeT0iNTM1IiBmb250LWZhbWlseT0iQXJpYWwiIGZvbnQtc2l6ZT0iMTQiIGZvbnQtd2VpZ2h0PSJib2xkIiBmaWxsPSIjMzQ0OTVlIj5UYXggKDE4JSk6PC90ZXh0PgogIDx0ZXh0IHg9IjY4MCIgeT0iNTM1IiBmb250LWZhbWlseT0iQXJpYWwiIGZvbnQtc2l6ZT0iMTQiIGZpbGw9IiMzNDQ5NWUiPiQxNDQuMDA8L3RleHQ+CiAgPHRleHQgeD0iNTUwIiB5PSI1NzAiIGZvbnQtZmFtaWx5PSJBcmlhbCIgZm9udC1zaXplPSIxNiIgZm9udC13ZWlnaHQ9ImJvbGQiIGZpbGw9IiMyYzNlNTAiPlRvdGFsOjwvdGV4dD4KICA8dGV4dCB4PSI2ODAiIHk9IjU3MCIgZm9udC1mYW1pbHk9IkFyaWFsIiBmb250LXNpemU9IjE2IiBmb250LXdlaWdodD0iYm9sZCIgZmlsbD0iIzJjM2U1MCI+JDk0NC4wMDwvdGV4dD4KCiAgPCEtLSBQYXltZW50IFRlcm1zIC0tPgogIDx0ZXh0IHg9IjQwIiB5PSI2NDAiIGZvbnQtZmFtaWx5PSJBcmlhbCIgZm9udC1zaXplPSIxNiIgZm9udC13ZWlnaHQ9ImJvbGQiIGZpbGw9IiMyYzNlNTAiPlBheW1lbnQgVGVybXM8L3RleHQ+CiAgPHRleHQgeD0iNDAiIHk9IjY3MCIgZm9udC1mYW1pbHk9IkFyaWFsIiBmb250LXNpemU9IjE0IiBmaWxsPSIjMzQ0OTVlIj5QYXltZW50IGlzIGR1ZSB3aXRoaW4gMzAgZGF5czwvdGV4dD4KICA8dGV4dCB4PSI0MCIgeT0iNjkwIiBmb250LWZhbWlseT0iQXJpYWwiIGZvbnQtc2l6ZT0iMTQiIGZpbGw9IiMzNDQ5NWUiPlBsZWFzZSBpbmNsdWRlIGludm9pY2UgbnVtYmVyIG9uIHBheW1lbnQ8L3RleHQ+CgogIDwhLS0gQmFuayBEZXRhaWxzIC0tPgogIDx0ZXh0IHg9IjQwIiB5PSI3MzAiIGZvbnQtZmFtaWx5PSJBcmlhbCIgZm9udC1zaXplPSIxNiIgZm9udC13ZWlnaHQ9ImJvbGQiIGZpbGw9IiMyYzNlNTAiPkJhbmsgRGV0YWlsczwvdGV4dD4KICA8dGV4dCB4PSI0MCIgeT0iNzYwIiBmb250LWZhbWlseT0iQXJpYWwiIGZvbnQtc2l6ZT0iMTQiIGZpbGw9IiMzNDQ5NWUiPkJhbms6IEJhbmsgb2YgR2VvcmdpYTwvdGV4dD4KICA8dGV4dCB4PSI0MCIgeT0iNzgwIiBmb250LWZhbWlseT0iQXJpYWwiIGZvbnQtc2l6ZT0iMTQiIGZpbGw9IiMzNDQ5NWUiPklCQU46IEdFMTIzNDU2Nzg5MDEyMzQ1Njc4PC90ZXh0PgogIDx0ZXh0IHg9IjQwIiB5PSI4MDAiIGZvbnQtZmFtaWx5PSJBcmlhbCIgZm9udC1zaXplPSIxNCIgZmlsbD0iIzM0NDk1ZSI+U1dJRlQ6IEJBR0FHRTIyPC90ZXh0PgoKICA8IS0tIEZvb3RlciAtLT4KICA8bGluZSB4MT0iNDAiIHkxPSI5MDAiIHgyPSI3NjAiIHkyPSI5MDAiIHN0cm9rZT0iIzJjM2U1MCIgc3Ryb2tlLXdpZHRoPSIxIi8+CiAgPHRleHQgeD0iNDAiIHk9IjkzMCIgZm9udC1mYW1pbHk9IkFyaWFsIiBmb250LXNpemU9IjEyIiBmaWxsPSIjN2Y4YzhkIj5UaGFuayB5b3UgZm9yIGNob29zaW5nIEdydXp5YSBBZHZlbnR1cmUgT3V0Zml0dGVyczwvdGV4dD4KICA8dGV4dCB4PSI0MCIgeT0iOTUwIiBmb250LWZhbWlseT0iQXJpYWwiIGZvbnQtc2l6ZT0iMTIiIGZpbGw9IiM3ZjhjOGQiPnd3dy5ncnV6eWFhZHZlbnR1cmVzLmNvbTwvdGV4dD4KPC9zdmc+Cg==".to_string()),
        filename: Some("logo.jpg".to_string()),
        ftype: Some(FileContentFtype::Jpg),
        furl: Some("".to_string())
        }),
        redirect_after_approve: Some(true),
        redirect_after_approve_url: Some("https://example.com/success".to_string())
        })
        },
        amount_fixed: None
        }, None).await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "POST",
        "/PaymentLink/23548884",
        Some(HashMap::from([(
            "mail2".to_string(),
            "jo@example.com; ceo@example.com".to_string(),
        )])),
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_payment_link_add_pay_link_from_bill_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .payment_link
        .add_pay_link_from_bill(
            23548884,
            &AddPayLinkFromBillRequest {
                mail_2: Some("jo@example.com; ceo@example.com".to_string()),
                body: PaymentPageRequestBody {
                    contact_us: Some(ContactElement {
                        email_label: Some("Email".to_string()),
                        enabled: Some(Enabled(true)),
                        header: Some("Contact Us".to_string()),
                        order: Some(Order(0)),
                        payment_icons: Some(true),
                        phone_label: Some("Phone".to_string()),
                    }),
                    invoices: None,
                    logo: Some(Element {
                        enabled: Some(Enabled(true)),
                        order: Some(Order(0)),
                    }),
                    message_before_paying: Some(LabelElement {
                        enabled: Some(Enabled(true)),
                        label: Some("Please review your payment details".to_string()),
                        order: Some(Order(0)),
                    }),
                    notes: Some(NoteElement {
                        enabled: Some(Enabled(true)),
                        header: Some("Additional Notes".to_string()),
                        order: Some(Order(0)),
                        placeholder: Some("Enter any additional notes here".to_string()),
                        value: Some("".to_string()),
                    }),
                    page: Some(PageElement {
                        description: Some("Get paid securely".to_string()),
                        enabled: Some(Enabled(true)),
                        header: Some("Payment Page".to_string()),
                        order: Some(Order(0)),
                    }),
                    payment_button: Some(LabelElement {
                        enabled: Some(Enabled(true)),
                        label: Some("Pay Now".to_string()),
                        order: Some(Order(0)),
                    }),
                    payment_methods: Some(MethodElement {
                        all_methods_checked: Some(true),
                        enabled: Some(Enabled(true)),
                        header: Some("Payment Methods".to_string()),
                        methods: Some(MethodsList {
                            amex: Some(true),
                            apple_pay: Some(true),
                            google_pay: None,
                            discover: Some(true),
                            e_check: Some(true),
                            mastercard: Some(true),
                            visa: Some(true),
                        }),
                        order: Some(Order(0)),
                        settings: None,
                    }),
                    payor: Some(PayorElement {
                        enabled: Some(Enabled(true)),
                        fields: Some(vec![PayorFields {
                            display: Some(true),
                            fixed: Some(true),
                            identifier: Some(true),
                            label: Some("Full Name".to_string()),
                            name: Some("fullName".to_string()),
                            order: Some(Order(0)),
                            required: Some(true),
                            validation: Some("alpha".to_string()),
                            value: Some("".to_string()),
                            width: Some(0),
                        }]),
                        header: Some("Payor Information".to_string()),
                        order: Some(Order(0)),
                    }),
                    review: Some(HeaderElement {
                        enabled: Some(Enabled(true)),
                        header: Some("Review Payment".to_string()),
                        order: Some(Order(0)),
                    }),
                    settings: Some(PagelinkSetting {
                        color: Some("#000000".to_string()),
                        custom_css_url: None,
                        language: Some("en".to_string()),
                        page_logo: None,
                        redirect_after_approve: None,
                        redirect_after_approve_url: None,
                    }),
                },
                amount_fixed: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "POST",
        "/PaymentLink/bill/23548884",
        Some(HashMap::from([(
            "mail2".to_string(),
            "jo@example.com; ceo@example.com".to_string(),
        )])),
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_payment_link_delete_pay_link_from_id_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .payment_link
        .delete_pay_link_from_id(&"payLinkId".to_string(), None)
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("DELETE", "/PaymentLink/payLinkId", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_payment_link_get_pay_link_from_id_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .payment_link
        .get_pay_link_from_id(&"paylinkId".to_string(), None)
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("GET", "/PaymentLink/load/paylinkId", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_payment_link_push_pay_link_from_id_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .payment_link
        .push_pay_link_from_id(
            &"payLinkId".to_string(),
            &PushPayLinkRequest::Sms {
                data: PushPayLinkRequestSms {},
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("POST", "/PaymentLink/push/payLinkId", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_payment_link_refresh_pay_link_from_id_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .payment_link
        .refresh_pay_link_from_id(
            &"payLinkId".to_string(),
            &RefreshPayLinkFromIdQueryRequest { amount_fixed: None },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("GET", "/PaymentLink/refresh/payLinkId", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_payment_link_send_pay_link_from_id_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .payment_link
        .send_pay_link_from_id(
            &"payLinkId".to_string(),
            &SendPayLinkFromIdQueryRequest {
                mail_2: Some("jo@example.com; ceo@example.com".to_string()),
                attachfile: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/PaymentLink/send/payLinkId",
        Some(HashMap::from([(
            "mail2".to_string(),
            "jo@example.com; ceo@example.com".to_string(),
        )])),
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_payment_link_update_pay_link_from_id_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .payment_link
        .update_pay_link_from_id(
            &"332-c277b704-1301".to_string(),
            &PayLinkUpdateData {
                notes: Some(NoteElement {
                    enabled: Some(Enabled(true)),
                    header: Some("Additional Notes".to_string()),
                    order: Some(Order(0)),
                    placeholder: Some("Enter any additional notes here".to_string()),
                    value: Some("".to_string()),
                }),
                payment_button: Some(LabelElement {
                    enabled: Some(Enabled(true)),
                    label: Some("Pay Now".to_string()),
                    order: Some(Order(0)),
                }),
                contact_us: None,
                logo: None,
                message_before_paying: None,
                page: None,
                payment_methods: None,
                review: None,
                settings: None,
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("PUT", "/PaymentLink/update/332-c277b704-1301", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_payment_link_add_pay_link_from_bill_lot_number_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::WIREMOCK_BASE_URL;

    let mut config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = ApiClient::new(config).expect("Failed to build client");

    let result = client
        .payment_link
        .add_pay_link_from_bill_lot_number(
            &"LOT-2024-001".to_string(),
            &AddPayLinkFromBillLotNumberRequest {
                entry_point: Entry("billing".to_string()),
                vendor_number: "VENDOR-123".to_string(),
                mail_2: Some("customer@example.com; billing@example.com".to_string()),
                amount_fixed: Some("true".to_string()),
                body: PaymentPageRequestBody {
                    contact_us: Some(ContactElement {
                        email_label: Some("Email".to_string()),
                        enabled: Some(Enabled(true)),
                        header: Some("Contact Us".to_string()),
                        order: Some(Order(0)),
                        payment_icons: Some(true),
                        phone_label: Some("Phone".to_string()),
                    }),
                    invoices: None,
                    logo: Some(Element {
                        enabled: Some(Enabled(true)),
                        order: Some(Order(0)),
                    }),
                    message_before_paying: Some(LabelElement {
                        enabled: Some(Enabled(true)),
                        label: Some("Please review your payment details".to_string()),
                        order: Some(Order(0)),
                    }),
                    notes: Some(NoteElement {
                        enabled: Some(Enabled(true)),
                        header: Some("Additional Notes".to_string()),
                        order: Some(Order(0)),
                        placeholder: Some("Enter any additional notes here".to_string()),
                        value: Some("".to_string()),
                    }),
                    page: Some(PageElement {
                        description: Some("Get paid securely".to_string()),
                        enabled: Some(Enabled(true)),
                        header: Some("Payment Page".to_string()),
                        order: Some(Order(0)),
                    }),
                    payment_button: Some(LabelElement {
                        enabled: Some(Enabled(true)),
                        label: Some("Pay Now".to_string()),
                        order: Some(Order(0)),
                    }),
                    payment_methods: Some(MethodElement {
                        all_methods_checked: Some(true),
                        enabled: Some(Enabled(true)),
                        header: Some("Payment Methods".to_string()),
                        methods: Some(MethodsList {
                            amex: Some(true),
                            apple_pay: Some(true),
                            google_pay: None,
                            discover: Some(true),
                            e_check: Some(true),
                            mastercard: Some(true),
                            visa: Some(true),
                        }),
                        order: Some(Order(0)),
                        settings: None,
                    }),
                    payor: Some(PayorElement {
                        enabled: Some(Enabled(true)),
                        fields: Some(vec![PayorFields {
                            display: Some(true),
                            fixed: Some(true),
                            identifier: Some(true),
                            label: Some("Full Name".to_string()),
                            name: Some("fullName".to_string()),
                            order: Some(Order(0)),
                            required: Some(true),
                            validation: Some("alpha".to_string()),
                            value: Some("".to_string()),
                            width: Some(0),
                        }]),
                        header: Some("Payor Information".to_string()),
                        order: Some(Order(0)),
                    }),
                    review: Some(HeaderElement {
                        enabled: Some(Enabled(true)),
                        header: Some("Review Payment".to_string()),
                        order: Some(Order(0)),
                    }),
                    settings: Some(PagelinkSetting {
                        color: Some("#000000".to_string()),
                        custom_css_url: None,
                        language: Some("en".to_string()),
                        page_logo: None,
                        redirect_after_approve: None,
                        redirect_after_approve_url: None,
                    }),
                },
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "POST",
        "/PaymentLink/bill/lotNumber/LOT-2024-001",
        Some(HashMap::from([
            ("entryPoint".to_string(), "billing".to_string()),
            ("vendorNumber".to_string(), "VENDOR-123".to_string()),
            (
                "mail2".to_string(),
                "customer@example.com; billing@example.com".to_string(),
            ),
            ("amountFixed".to_string(), "true".to_string()),
        ])),
        1,
    )
    .await
    .unwrap();
}
