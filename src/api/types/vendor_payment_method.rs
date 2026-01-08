pub use crate::prelude::*;

/// Payment method object to use for the payout.
/// - `{ method: "managed" }` - Managed payment method
/// - `{ method: "vcard" }` - Virtual card payment method
/// - `{ method: "check" }` - Check payment method
/// - `{ method: "ach", storedMethodId?: "..." }` - ACH payment method with optional stored method ID
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct VendorPaymentMethod {
    /// Payment method type - "managed", "vcard", "check", or "ach"
    pub method: String,
    /// ID of the stored ACH payment method. Only applicable when method is "ach". Required when using a previously saved ACH method when the vendor has more than one saved method. See the [Payouts with saved ACH payment methods](/developers/developer-guides/pay-out-manage-payouts) section for more details.
    #[serde(rename = "storedMethodId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_method_id: Option<String>,
}
