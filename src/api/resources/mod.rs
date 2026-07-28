//! Service clients and API endpoints
//!
//! This module contains client implementations for:
//!
//! - **Bill**
//! - **Customer**
//! - **CheckCapture**
//! - **MoneyIn**
//! - **Token**
//! - **Subscription**
//! - **Invoice**
//! - **PaymentLink**
//! - **TokenStorage**
//! - **Paypoint**
//! - **HostedPaymentPages**
//! - **PaymentMethodDomain**
//! - **Import**
//! - **Query**
//! - **Ocr**
//! - **Notificationlogs**
//! - **Cloud**
//! - **LineItem**
//! - **Boarding**
//! - **Templates**
//! - **Export**
//! - **Organization**
//! - **Management**
//! - **Statistic**
//! - **Notification**
//! - **User**
//! - **Vendor**
//! - **GhostCard**
//! - **MoneyOut**
//! - **Funding**
//! - **Wallet**
//! - **PayoutSubscription**
//! - **ChargeBacks**
//! - **Case Management**

use crate::{ApiError, ClientConfig};

pub mod bill;
pub mod boarding;
pub mod case_management;
pub mod charge_backs;
pub mod check_capture;
pub mod cloud;
pub mod customer;
pub mod export;
pub mod funding;
pub mod ghost_card;
pub mod hosted_payment_pages;
pub mod import;
pub mod invoice;
pub mod line_item;
pub mod management;
pub mod money_in;
pub mod money_out;
pub mod notification;
pub mod notificationlogs;
pub mod ocr;
pub mod organization;
pub mod payment_link;
pub mod payment_method_domain;
pub mod payout_subscription;
pub mod paypoint;
pub mod query;
pub mod statistic;
pub mod subscription;
pub mod templates;
pub mod token;
pub mod token_storage;
pub mod user;
pub mod vendor;
pub mod wallet;
pub struct ApiClient {
    pub config: ClientConfig,
    pub bill: BillClient,
    pub customer: CustomerClient,
    pub check_capture: CheckCaptureClient,
    pub money_in: MoneyInClient,
    pub token: TokenClient,
    pub subscription: SubscriptionClient,
    pub invoice: InvoiceClient,
    pub payment_link: PaymentLinkClient,
    pub token_storage: TokenStorageClient,
    pub paypoint: PaypointClient,
    pub hosted_payment_pages: HostedPaymentPagesClient,
    pub payment_method_domain: PaymentMethodDomainClient,
    pub import: ImportClient,
    pub query: QueryClient,
    pub ocr: OcrClient,
    pub notificationlogs: NotificationlogsClient,
    pub cloud: CloudClient,
    pub line_item: LineItemClient,
    pub boarding: BoardingClient,
    pub templates: TemplatesClient,
    pub export: ExportClient,
    pub organization: OrganizationClient,
    pub management: ManagementClient,
    pub statistic: StatisticClient,
    pub notification: NotificationClient,
    pub user: UserClient,
    pub vendor: VendorClient,
    pub ghost_card: GhostCardClient,
    pub money_out: MoneyOutClient,
    pub funding: FundingClient,
    pub wallet: WalletClient,
    pub payout_subscription: PayoutSubscriptionClient,
    pub charge_backs: ChargeBacksClient,
    pub case_management: CaseManagementClient,
}

impl ApiClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            config: config.clone(),
            bill: BillClient::new(config.clone())?,
            customer: CustomerClient::new(config.clone())?,
            check_capture: CheckCaptureClient::new(config.clone())?,
            money_in: MoneyInClient::new(config.clone())?,
            token: TokenClient::new(config.clone())?,
            subscription: SubscriptionClient::new(config.clone())?,
            invoice: InvoiceClient::new(config.clone())?,
            payment_link: PaymentLinkClient::new(config.clone())?,
            token_storage: TokenStorageClient::new(config.clone())?,
            paypoint: PaypointClient::new(config.clone())?,
            hosted_payment_pages: HostedPaymentPagesClient::new(config.clone())?,
            payment_method_domain: PaymentMethodDomainClient::new(config.clone())?,
            import: ImportClient::new(config.clone())?,
            query: QueryClient::new(config.clone())?,
            ocr: OcrClient::new(config.clone())?,
            notificationlogs: NotificationlogsClient::new(config.clone())?,
            cloud: CloudClient::new(config.clone())?,
            line_item: LineItemClient::new(config.clone())?,
            boarding: BoardingClient::new(config.clone())?,
            templates: TemplatesClient::new(config.clone())?,
            export: ExportClient::new(config.clone())?,
            organization: OrganizationClient::new(config.clone())?,
            management: ManagementClient::new(config.clone())?,
            statistic: StatisticClient::new(config.clone())?,
            notification: NotificationClient::new(config.clone())?,
            user: UserClient::new(config.clone())?,
            vendor: VendorClient::new(config.clone())?,
            ghost_card: GhostCardClient::new(config.clone())?,
            money_out: MoneyOutClient::new(config.clone())?,
            funding: FundingClient::new(config.clone())?,
            wallet: WalletClient::new(config.clone())?,
            payout_subscription: PayoutSubscriptionClient::new(config.clone())?,
            charge_backs: ChargeBacksClient::new(config.clone())?,
            case_management: CaseManagementClient::new(config.clone())?,
        })
    }
}

pub use bill::BillClient;
pub use boarding::BoardingClient;
pub use case_management::CaseManagementClient;
pub use charge_backs::ChargeBacksClient;
pub use check_capture::CheckCaptureClient;
pub use cloud::CloudClient;
pub use customer::CustomerClient;
pub use export::ExportClient;
pub use funding::FundingClient;
pub use ghost_card::GhostCardClient;
pub use hosted_payment_pages::HostedPaymentPagesClient;
pub use import::ImportClient;
pub use invoice::InvoiceClient;
pub use line_item::LineItemClient;
pub use management::ManagementClient;
pub use money_in::MoneyInClient;
pub use money_out::MoneyOutClient;
pub use notification::NotificationClient;
pub use notificationlogs::NotificationlogsClient;
pub use ocr::OcrClient;
pub use organization::OrganizationClient;
pub use payment_link::PaymentLinkClient;
pub use payment_method_domain::PaymentMethodDomainClient;
pub use payout_subscription::PayoutSubscriptionClient;
pub use paypoint::PaypointClient;
pub use query::QueryClient;
pub use statistic::StatisticClient;
pub use subscription::SubscriptionClient;
pub use templates::TemplatesClient;
pub use token::TokenClient;
pub use token_storage::TokenStorageClient;
pub use user::UserClient;
pub use vendor::VendorClient;
pub use wallet::WalletClient;
