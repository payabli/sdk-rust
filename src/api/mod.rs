//! API client and types for the Payabli API
//!
//! This module contains all the API definitions including request/response types
//! and client implementations for interacting with the API.
//!
//! ## Modules
//!
//! - [`resources`] - Service clients and endpoints
//! - [`types`] - Request, response, and model types

pub mod resources;
pub mod types;

pub use resources::{
    ApiClient, BillClient, BoardingClient, ChargeBacksClient, CheckCaptureClient, CloudClient,
    CustomerClient, ExportClient, FundingClient, GhostCardClient, HostedPaymentPagesClient,
    ImportClient, InvoiceClient, LineItemClient, ManagementClient, MoneyInClient, MoneyOutClient,
    NotificationClient, NotificationlogsClient, OcrClient, OrganizationClient, PaymentLinkClient,
    PaymentMethodDomainClient, PayoutSubscriptionClient, PaypointClient, QueryClient,
    StatisticClient, SubscriptionClient, TemplatesClient, TokenClient, TokenStorageClient,
    UserClient, VendorClient, WalletClient,
};
pub use types::*;
