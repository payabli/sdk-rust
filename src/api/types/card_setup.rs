pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CardSetup {
    /// Determines whether American Express is accepted.
    #[serde(rename = "acceptAmex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_amex: Option<bool>,
    /// Determines whether Discover is accepted.
    #[serde(rename = "acceptDiscover")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_discover: Option<bool>,
    /// Determines whether Mastercard is accepted.
    #[serde(rename = "acceptMastercard")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_mastercard: Option<bool>,
    /// Determines whether Visa is accepted.
    #[serde(rename = "acceptVisa")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_visa: Option<bool>,
}
