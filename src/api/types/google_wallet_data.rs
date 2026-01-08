pub use crate::prelude::*;

/// The wallet data.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GoogleWalletData {
    /// The Google Pay merchant identifier.
    #[serde(rename = "gatewayMerchantId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_merchant_id: Option<String>,
    /// The Google Pay gateway identifier.
    #[serde(rename = "gatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_id: Option<String>,
}
