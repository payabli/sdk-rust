pub use crate::prelude::*;

/// Payment method for reissuing a payout transaction. The reissue endpoint uses the payment method details directly. It doesn't fall back to the vendor's managed payment method.
/// - `{ method: "vcard" }` - Reissue as a virtual card
/// - `{ method: "check" }` - Reissue as a paper check
/// - `{ method: "ach", achHolder: "...", achRouting: "...", achAccount: "...", achAccountType: "...", achHolderType: "..." }` - Reissue as ACH with bank details
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ReissuePaymentMethod {
    /// Payment method type. Must be `"ach"`, `"check"`, or `"vcard"`.
    pub method: String,
    /// Account holder name. Required when `method` is `"ach"`.
    #[serde(rename = "achHolder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_holder: Option<String>,
    /// Bank routing number (9 digits). Required when `method` is `"ach"`.
    #[serde(rename = "achRouting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_routing: Option<String>,
    /// Bank account number (8-17 digits). Required when `method` is `"ach"`.
    #[serde(rename = "achAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_account: Option<String>,
    /// Bank account type (`"checking"` or `"savings"`). Required when `method` is `"ach"`.
    #[serde(rename = "achAccountType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_account_type: Option<String>,
    #[serde(rename = "achHolderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_holder_type: Option<AchHolderType>,
}
