pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SettingsQueryRecord {
    /// Any custom fields defined for the org.
    #[serde(rename = "customFields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<KeyValue>>,
    #[serde(rename = "forInvoices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub for_invoices: Option<Vec<KeyValue>>,
    #[serde(rename = "forPayOuts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub for_pay_outs: Option<Vec<KeyValue>>,
    /// Information about digital wallet settings for the entity. Available values are `isApplePayEnabled` and `isGooglePayEnabled`.
    #[serde(rename = "forWallets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub for_wallets: Option<Vec<KeyValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general: Option<Vec<KeyValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<Vec<KeyValue>>,
}
