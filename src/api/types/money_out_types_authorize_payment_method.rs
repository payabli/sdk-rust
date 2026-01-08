pub use crate::prelude::*;

/// Payment method object for vendor payouts.
/// - `{ method: "managed" }` - Managed payment method
/// - `{ method: "vcard" }` - Virtual card payment method
/// - `{ method: "check" }` - Check payment method
/// - `{ method: "ach", achHolder: "...", achRouting: "...", achAccount: "...", achAccountType: "..." }` - ACH payment method with bank details
/// - `{ method: "ach", storedMethodId: "..." }` - ACH payment method using stored method ID
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AuthorizePaymentMethod {
    /// Payment method type - "managed", "vcard", "check", or "ach"
    pub method: String,
    /// Account holder name for ACH payments. Required when method is "ach" and not using `storedMethodId`.
    #[serde(rename = "achHolder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_holder: Option<String>,
    /// Bank routing number for ACH payments. Required when method is "ach" and not using `storedMethodId`.
    #[serde(rename = "achRouting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_routing: Option<String>,
    /// Bank account number for ACH payments. Required when method is "ach" and not using `storedMethodId`.
    #[serde(rename = "achAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_account: Option<String>,
    /// Account type for ACH payments ("checking" or "savings"). Required when method is "ach" and not using `storedMethodId`.
    #[serde(rename = "achAccountType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_account_type: Option<String>,
    #[serde(rename = "achCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_code: Option<AchSecCode>,
    #[serde(rename = "achHolderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_holder_type: Option<AchHolderType>,
    /// ID of the stored ACH payment method. Only applicable when method is `ach`. Use this to reference a previously saved ACH method instead of providing bank details directly.
    #[serde(rename = "storedMethodId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_method_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiator: Option<Initiator>,
    #[serde(rename = "storedMethodUsageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_method_usage_type: Option<StoredMethodUsageType>,
}
