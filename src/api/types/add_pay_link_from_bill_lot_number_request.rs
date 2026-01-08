pub use crate::prelude::*;

/// Request for AddPayLinkFromBillLotNumber (body + query parameters)
///
/// Request type for the AddPayLinkFromBillLotNumberRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AddPayLinkFromBillLotNumberRequest {
    #[serde(rename = "entryPoint")]
    pub entry_point: Entry,
    /// The vendor number for the vendor being paid with this payment link.
    #[serde(rename = "vendorNumber")]
    pub vendor_number: String,
    /// List of recipient email addresses. When there is more than one, separate them by a semicolon (;).
    #[serde(rename = "mail2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail_2: Option<String>,
    /// Indicates whether customer can modify the payment amount. A value of `true` means the amount isn't modifiable, a value `false` means the payor can modify the amount to pay.
    #[serde(rename = "amountFixed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_fixed: Option<String>,
    pub body: PaymentPageRequestBody,
}
