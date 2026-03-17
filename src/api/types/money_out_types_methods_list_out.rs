pub use crate::prelude::*;

/// Payment methods available for Pay Out payment links. Controls which payout options are offered to the vendor.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MethodsListOut {
    /// When `true`, ACH bank transfer is offered as a payout method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<bool>,
    /// When `true`, physical check is offered as a payout method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check: Option<bool>,
    /// When `true`, virtual card (vCard) is offered as a payout method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<bool>,
}
