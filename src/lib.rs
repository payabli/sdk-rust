//! # Payabli API SDK
//!
//! The official Rust SDK for the Payabli API.
//!
//! ## Getting Started
//!
//! ```rust
//! use payabli_api::prelude::*;
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
//!                 attachments: Some(Attachments(vec![FileContent {
//!                     filename: Some("my-doc.pdf".to_string()),
//!                     ftype: Some(FileContentFtype::Pdf),
//!                     furl: Some("https://mysite.com/my-doc.pdf".to_string()),
//!                     ..Default::default()
//!                 }])),
//!                 bill_date: Some(NaiveDate::parse_from_str("2024-07-01", "%Y-%m-%d").unwrap()),
//!                 bill_items: Some(Billitems(vec![BillItem {
//!                     item_categories: Some(vec!["deposits".to_string()]),
//!                     item_commodity_code: Some(ItemCommodityCode("010".to_string())),
//!                     item_cost: Some(5.0),
//!                     item_description: Some(ItemDescription("Deposit for materials".to_string())),
//!                     item_mode: Some(0),
//!                     item_product_code: Some(ItemProductCode("M-DEPOSIT".to_string())),
//!                     item_product_name: Some(ItemProductName("Materials deposit".to_string())),
//!                     item_qty: Some(1),
//!                     item_tax_amount: Some(7.0),
//!                     item_tax_rate: Some(0.075),
//!                     item_total_amount: Some(123.0),
//!                     item_unit_of_measure: Some(ItemUnitofMeasure("SqFt".to_string())),
//!                     ..Default::default()
//!                 }])),
//!                 bill_number: Some("ABC-123".to_string()),
//!                 comments: Some(Comments("Deposit for materials".to_string())),
//!                 due_date: Some(NaiveDate::parse_from_str("2024-07-01", "%Y-%m-%d").unwrap()),
//!                 end_date: Some(NaiveDate::parse_from_str("2024-07-01", "%Y-%m-%d").unwrap()),
//!                 frequency: Some(Frequency::Monthly),
//!                 mode: Some(0),
//!                 net_amount: Some(3762.87),
//!                 status: Some(Billstatus(1)),
//!                 terms: Some(Terms::Net30),
//!                 vendor: Some(BillOutDataVendor {
//!                     vendor_number: Some(VendorNumber("VEN-123".to_string())),
//!                     ..Default::default()
//!                 }),
//!                 ..Default::default()
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
pub use error::{ApiError, BuildError};
