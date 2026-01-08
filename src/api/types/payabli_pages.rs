pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PayabliPages {
    #[serde(rename = "AdditionalData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<AdditionalData>,
    /// Array of credential objects with active services for the page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Vec<PayabliCredentials>>,
    /// Timestamp of last access to page structure
    #[serde(rename = "lastAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub last_access: Option<DateTime<FixedOffset>>,
    /// Sections of page
    #[serde(rename = "pageContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_content: Option<PageContent>,
    #[serde(rename = "pageIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_identifier: Option<PageIdentifier>,
    /// Settings of page
    #[serde(rename = "pageSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_settings: Option<PageSetting>,
    /// Flag indicating if page is active to accept payments. `0` for false, `1` for true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published: Option<i64>,
    /// Sections of payment receipt
    #[serde(rename = "receiptContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_content: Option<ReceiptContent>,
    /// Page identifier. Must be unique in platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdomain: Option<Subdomain>,
    /// Total amount to pay in this page
    #[serde(rename = "totalAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<f64>,
    /// Base64 encoded image of CAPTCHA associated to this page load
    #[serde(rename = "validationCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_code: Option<String>,
}
