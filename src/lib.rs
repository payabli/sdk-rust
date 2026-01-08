//! # Payabli API SDK
//!
//!
//! ## Payabli API
//!
//! The Payabli API is a RESTful API that enables you to build robust payment solutions on the Payabli platform. Process card and ACH transactions, manage entities like vendors and customers, handle subscriptions and recurring billing, tokenize payment methods securely, and send payouts.
//!
//! The API provides comprehensive endpoints for accepting payments, making payouts to vendors, and managing merchant operations. Use our sandbox environment to build and test your integration, then seamlessly move to production.
//!
//! The API includes built-in security features like idempotency keys to prevent duplicate transactions, rate limits for optimal performance, and multiple authentication token types for different use cases. Available in both sandbox and production environments with complete support for webhooks, fraud controls, and real-time transaction processing.
//!
//! See the [API overview](https://docs.payabli.com/developers/api-reference/api-overview) for information on authentication, error handling, idempotency, rate limits, and more.
//!
//! See our full collection of [developer tools](https://docs.payabli.com/developers/developer-guides/tools-overview) for SDKs, our MCP server, Postman collection information, and example apps to help you get started quickly.
//!
//! ## Getting Started
//!
//! ```rust
//! use payabli_api::prelude::*;
//! use payabli_api::{
//!     AccountingField, AdditionalData, AdditionalDataString, AddressAddtlNullable, AddressNullable,
//!     Attachments, BankAccountHolderName, BankAccountHolderType, BankName, BillItem, BillOutData,
//!     BillOutDataScheduledOptions, BillingData, Billitems, Billstatus, Comments, Contacts,
//!     ContactsField, Datenullable, Email, FileContent, FileContentFtype, Frequency, IdempotencyKey,
//!     ItemCommodityCode, ItemDescription, ItemProductCode, ItemProductName, ItemUnitofMeasure,
//!     LocationCode, Mcc, PayeeName, RemitEmail, Remitaddress1, Remitaddress2, Remitcity,
//!     Remitcountry, Remitstate, Remitzip, RoutingAccount, Terms, TypeAccount, VendorData, VendorEin,
//!     VendorName1, VendorName2, VendorNumber, VendorPaymentMethodString, VendorPhone, Vendorstatus,
//! };
//!
//! #[tokio::main]
//! async fn main() {
//!     let config = ClientConfig {
//!         api_key: Some("<value>".to_string()),
//!         ..Default::default()
//!     };
//!     let client = ApiClient::new(config).expect("Failed to build client");
//!     client
//!         .bill
//!         .add_bill(
//!             &"8cfec329267".to_string(),
//!             &BillOutData {
//!                 accounting_field_1: Some(AccountingField("MyInternalId".to_string())),
//!                 accounting_field_2: None,
//!                 additional_data: None,
//!                 attachments: Some(Attachments(Some(vec![FileContent {
//!                     f_content: None,
//!                     filename: Some("my-doc.pdf".to_string()),
//!                     ftype: Some(FileContentFtype::Pdf),
//!                     furl: Some("https://mysite.com/my-doc.pdf".to_string()),
//!                 }]))),
//!                 bill_date: Some(Datenullable(Some(
//!                     NaiveDate::parse_from_str("2024-07-01", "%Y-%m-%d").unwrap(),
//!                 ))),
//!                 bill_items: Some(Billitems(Some(vec![BillItem {
//!                     item_categories: Some(vec![Some("deposits".to_string())]),
//!                     item_commodity_code: Some(ItemCommodityCode("010".to_string())),
//!                     item_cost: 5.0,
//!                     item_description: Some(ItemDescription("Deposit for materials".to_string())),
//!                     item_mode: Some(0),
//!                     item_product_code: Some(ItemProductCode("M-DEPOSIT".to_string())),
//!                     item_product_name: Some(ItemProductName("Materials deposit".to_string())),
//!                     item_qty: Some(1),
//!                     item_tax_amount: Some(7.0),
//!                     item_tax_rate: Some(0.075),
//!                     item_total_amount: Some(123.0),
//!                     item_unit_of_measure: Some(ItemUnitofMeasure("SqFt".to_string())),
//!                 }]))),
//!                 bill_number: Some("ABC-123".to_string()),
//!                 comments: Some(Comments("Deposit for materials".to_string())),
//!                 discount: None,
//!                 due_date: Some(Datenullable(Some(
//!                     NaiveDate::parse_from_str("2024-07-01", "%Y-%m-%d").unwrap(),
//!                 ))),
//!                 end_date: Some(Datenullable(Some(
//!                     NaiveDate::parse_from_str("2024-07-01", "%Y-%m-%d").unwrap(),
//!                 ))),
//!                 frequency: Some(Frequency::Monthly),
//!                 lot_number: None,
//!                 mode: Some(0),
//!                 net_amount: Some(3762.87),
//!                 scheduled_options: None,
//!                 status: Some(Billstatus(-99)),
//!                 terms: Some(Terms("NET30".to_string())),
//!                 total_amount: None,
//!                 vendor: Some(VendorData {
//!                     vendor_number: Some(VendorNumber("1234-A".to_string())),
//!                     additional_data: None,
//!                     address_1: None,
//!                     address_2: None,
//!                     billing_data: None,
//!                     city: None,
//!                     contacts: None,
//!                     country: None,
//!                     custom_field_1: None,
//!                     custom_field_2: None,
//!                     customer_vendor_account: None,
//!                     ein: None,
//!                     email: None,
//!                     internal_reference_id: None,
//!                     location_code: None,
//!                     mcc: None,
//!                     name_1: None,
//!                     name_2: None,
//!                     payee_name_1: None,
//!                     payee_name_2: None,
//!                     payment_method: None,
//!                     phone: None,
//!                     remit_address_1: None,
//!                     remit_address_2: None,
//!                     remit_city: None,
//!                     remit_country: None,
//!                     remit_email: None,
//!                     remit_state: None,
//!                     remit_zip: None,
//!                     state: None,
//!                     vendor_status: None,
//!                     zip: None,
//!                 }),
//!             },
//!             None,
//!         )
//!         .await;
//! }
//! ```
//!
//! ## Modules
//!
//! - [`api`] - Core API types and models
//! - [`client`] - Client implementations
//! - [`config`] - Configuration options
//! - [`core`] - Core utilities and infrastructure
//! - [`error`] - Error types and handling
//! - [`prelude`] - Common imports for convenience

pub mod api;
pub mod client;
pub mod config;
pub mod core;
pub mod environment;
pub mod error;
pub mod prelude;

pub use api::*;
pub use client::*;
pub use config::*;
pub use core::*;
pub use environment::*;
pub use error::ApiError;
