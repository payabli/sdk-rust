pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MethodsList {
    /// When `true`, American Express is accepted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amex: Option<bool>,
    /// When `true`, Apple Pay is accepted.
    #[serde(rename = "applePay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apple_pay: Option<bool>,
    /// When `true`, Google Pay is accepted.
    #[serde(rename = "googlePay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_pay: Option<bool>,
    /// When `true`, Discover is accepted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discover: Option<bool>,
    /// When `true`, ACH is accepted.
    #[serde(rename = "eCheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_check: Option<bool>,
    /// When `true`, Mastercard is accepted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mastercard: Option<bool>,
    /// When `true`, Visa is accepted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visa: Option<bool>,
}
