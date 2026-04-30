//! API client and types for the Payabli API
//!
//!
//! ## Payabli API
//!
//! ## Modules
//!
//! - [`resources`] - Service clients and endpoints
//! - [`types`] - Request, response, and model types

pub mod resources;
pub mod types;

pub use resources::{
    ApiClient, BillClient, BoardingClient, ChargeBacksClient, CheckCaptureClient, CloudClient,
    CustomerClient, ExportClient, GhostCardClient, HostedPaymentPagesClient, ImportClient,
    InvoiceClient, LineItemClient, ManagementClient, MoneyInClient, MoneyOutClient,
    MoneyOutTypesClient, NotificationClient, NotificationlogsClient, OcrClient, OrganizationClient,
    PaymentLinkClient, PaymentMethodDomainClient, PayoutSubscriptionClient, PaypointClient,
    QueryClient, QueryTypesClient, StatisticClient, SubscriptionClient, TemplatesClient,
    TokenStorageClient, UserClient, V2MoneyInTypesClient, VendorClient, WalletClient,
};
pub use types::*;
