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
    ApiClient, ApplicationsClient, BillClient, BillingClient, BoardingClient,
    BoardingTemplatesClient, BusinessesClient, CaseManagementClient, ChargeBacksClient,
    CheckCaptureClient, CloudClient, CustomerClient, DeviceClient, ExportClient, FundingClient,
    GhostCardClient, HostedPaymentPagesClient, ImportClient, InvoiceClient, LineItemClient,
    ManagementClient, MoneyInClient, MoneyOutClient, NotificationClient, NotificationlogsClient,
    OcrClient, OrganizationClient, PaymentLinkClient, PaymentMethodDomainClient,
    PaymentMethodsClient, PayoutSubscriptionClient, PaypointClient, PaypointsClient, PeopleClient,
    QueryClient, StatisticClient, SubscriptionClient, TaptopayClient, TemplatesClient, TokenClient,
    TokenStorageClient, UserClient, VendorClient, WalletClient,
};
pub use types::*;
