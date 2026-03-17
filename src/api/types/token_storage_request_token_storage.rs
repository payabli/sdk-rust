pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RequestTokenStorage {
    /// Object describing the Customer/Payor owner of payment method. Required for POST requests. Which fields are required depends on the paypoint's custom identifier settings.
    #[serde(rename = "customerData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_data: Option<PayorDataRequest>,
    /// Entrypoint identifier. Required for POST requests.
    #[serde(rename = "entryPoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_point: Option<Entrypointfield>,
    /// When `true`, if tokenization fails, Payabli will attempt an authorization transaction to request a permanent token for the card. If the authorization is successful, the card will be tokenized and the authorization will be voided automatically.
    #[serde(rename = "fallbackAuth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback_auth: Option<bool>,
    /// The amount for the `fallbackAuth` transaction. Defaults to one dollar (`100`).
    #[serde(rename = "fallbackAuthAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback_auth_amount: Option<i64>,
    /// Custom description for stored payment method.
    #[serde(rename = "methodDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method_description: Option<String>,
    /// Information about the payment method for the transaction.
    #[serde(rename = "paymentMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<RequestTokenStoragePaymentMethod>,
    #[serde(rename = "vendorData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_data: Option<VendorDataRequest>,
    /// Custom identifier to indicate the source for the request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdomain: Option<Subdomain>,
}
