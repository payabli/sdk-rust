pub use crate::prelude::*;

/// Object describing the vendor owner of payment method. Required when saving an ACH payment method on behalf of a vendor (for Pay Out transactions).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct VendorDataRequest {
    /// The unique numeric ID assigned to the vendor in Payabli. Either `vendorId` or `vendorNumber` is required.
    #[serde(rename = "vendorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_id: Option<i64>,
    /// Custom vendor number assigned by the business. Either `vendorId` or `vendorNumber` is required.
    #[serde(rename = "vendorNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_number: Option<String>,
}
