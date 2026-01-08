//! Service clients and API endpoints
//!
//! This module contains client implementations for:
//!
//! - **MoneyOutTypes**
//! - **QueryTypes**
//! - **V2MoneyInTypes**
//! - **Bill**
//! - **Boarding**
//! - **ChargeBacks**
//! - **CheckCapture**
//! - **Cloud**
//! - **Customer**
//! - **Export**
//! - **HostedPaymentPages**
//! - **Import**
//! - **Invoice**
//! - **LineItem**
//! - **MoneyIn**
//! - **MoneyOut**
//! - **Notification**
//! - **Notificationlogs**
//! - **Ocr**
//! - **Organization**
//! - **PaymentLink**
//! - **PaymentMethodDomain**
//! - **Paypoint**
//! - **Query**
//! - **Statistic**
//! - **Subscription**
//! - **Templates**
//! - **TokenStorage**
//! - **User**
//! - **Vendor**
//! - **Wallet**

use crate::{ApiError, ClientConfig};

pub mod bill;
pub mod boarding;
pub mod charge_backs;
pub mod check_capture;
pub mod cloud;
pub mod customer;
pub mod export;
pub mod hosted_payment_pages;
pub mod import;
pub mod invoice;
pub mod line_item;
pub mod money_in;
pub mod money_out;
pub mod money_out_types;
pub mod notification;
pub mod notificationlogs;
pub mod ocr;
pub mod organization;
pub mod payment_link;
pub mod payment_method_domain;
pub mod paypoint;
pub mod query;
pub mod query_types;
pub mod statistic;
pub mod subscription;
pub mod templates;
pub mod token_storage;
pub mod user;
pub mod v_2_money_in_types;
pub mod vendor;
pub mod wallet;
pub struct ApiClient {
    pub config: ClientConfig,
    pub bill: BillClient,
    pub boarding: BoardingClient,
    pub charge_backs: ChargeBacksClient,
    pub check_capture: CheckCaptureClient,
    pub cloud: CloudClient,
    pub customer: CustomerClient,
    pub export: ExportClient,
    pub hosted_payment_pages: HostedPaymentPagesClient,
    pub import: ImportClient,
    pub invoice: InvoiceClient,
    pub line_item: LineItemClient,
    pub money_in: MoneyInClient,
    pub money_out: MoneyOutClient,
    pub notification: NotificationClient,
    pub notificationlogs: NotificationlogsClient,
    pub ocr: OcrClient,
    pub organization: OrganizationClient,
    pub payment_link: PaymentLinkClient,
    pub payment_method_domain: PaymentMethodDomainClient,
    pub paypoint: PaypointClient,
    pub query: QueryClient,
    pub statistic: StatisticClient,
    pub subscription: SubscriptionClient,
    pub templates: TemplatesClient,
    pub token_storage: TokenStorageClient,
    pub user: UserClient,
    pub vendor: VendorClient,
    pub wallet: WalletClient,
}

impl ApiClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            config: config.clone(),
            bill: BillClient::new(config.clone())?,
            boarding: BoardingClient::new(config.clone())?,
            charge_backs: ChargeBacksClient::new(config.clone())?,
            check_capture: CheckCaptureClient::new(config.clone())?,
            cloud: CloudClient::new(config.clone())?,
            customer: CustomerClient::new(config.clone())?,
            export: ExportClient::new(config.clone())?,
            hosted_payment_pages: HostedPaymentPagesClient::new(config.clone())?,
            import: ImportClient::new(config.clone())?,
            invoice: InvoiceClient::new(config.clone())?,
            line_item: LineItemClient::new(config.clone())?,
            money_in: MoneyInClient::new(config.clone())?,
            money_out: MoneyOutClient::new(config.clone())?,
            notification: NotificationClient::new(config.clone())?,
            notificationlogs: NotificationlogsClient::new(config.clone())?,
            ocr: OcrClient::new(config.clone())?,
            organization: OrganizationClient::new(config.clone())?,
            payment_link: PaymentLinkClient::new(config.clone())?,
            payment_method_domain: PaymentMethodDomainClient::new(config.clone())?,
            paypoint: PaypointClient::new(config.clone())?,
            query: QueryClient::new(config.clone())?,
            statistic: StatisticClient::new(config.clone())?,
            subscription: SubscriptionClient::new(config.clone())?,
            templates: TemplatesClient::new(config.clone())?,
            token_storage: TokenStorageClient::new(config.clone())?,
            user: UserClient::new(config.clone())?,
            vendor: VendorClient::new(config.clone())?,
            wallet: WalletClient::new(config.clone())?,
        })
    }
}

pub use bill::BillClient;
pub use boarding::BoardingClient;
pub use charge_backs::ChargeBacksClient;
pub use check_capture::CheckCaptureClient;
pub use cloud::CloudClient;
pub use customer::CustomerClient;
pub use export::ExportClient;
pub use hosted_payment_pages::HostedPaymentPagesClient;
pub use import::ImportClient;
pub use invoice::InvoiceClient;
pub use line_item::LineItemClient;
pub use money_in::MoneyInClient;
pub use money_out::MoneyOutClient;
pub use money_out_types::MoneyOutTypesClient;
pub use notification::NotificationClient;
pub use notificationlogs::NotificationlogsClient;
pub use ocr::OcrClient;
pub use organization::OrganizationClient;
pub use payment_link::PaymentLinkClient;
pub use payment_method_domain::PaymentMethodDomainClient;
pub use paypoint::PaypointClient;
pub use query::QueryClient;
pub use query_types::QueryTypesClient;
pub use statistic::StatisticClient;
pub use subscription::SubscriptionClient;
pub use templates::TemplatesClient;
pub use token_storage::TokenStorageClient;
pub use user::UserClient;
pub use v_2_money_in_types::V2MoneyInTypesClient;
pub use vendor::VendorClient;
pub use wallet::WalletClient;
