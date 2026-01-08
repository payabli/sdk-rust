pub use crate::prelude::*;

/// Request for AuthorizeOut (body + query parameters)
///
/// Request type for the AuthorizeOutRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuthorizeOutRequest {
    /// When `true`, the authorization bypasses the requirement for unique bills, identified by vendor invoice number. This allows you to make more than one payout authorization for a bill, like a split payment.
    #[serde(rename = "allowDuplicatedBills")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_duplicated_bills: Option<bool>,
    /// When `true`, Payabli won't automatically create a bill for this payout transaction.
    #[serde(rename = "doNotCreateBills")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub do_not_create_bills: Option<bool>,
    /// When `true`, the request creates a new vendor record, regardless of whether the vendor already exists.
    #[serde(rename = "forceVendorCreation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_vendor_creation: Option<bool>,
    pub body: AuthorizePayoutBody,
}
