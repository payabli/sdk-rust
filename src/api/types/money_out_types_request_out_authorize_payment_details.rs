pub use crate::prelude::*;

/// Object containing payment details.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RequestOutAuthorizePaymentDetails {
    #[serde(rename = "checkNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_number: Option<VendorCheckNumber>,
    /// Currency code ISO-4217. If no code is provided, then the currency in the paypoint setting is used. Default is **USD**.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Service fee to be deducted from the total amount. This amount must be a number, percentages aren't accepted. If you are using a percentage-based fee schedule, you must calculate the value manually.
    #[serde(rename = "serviceFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_fee: Option<f64>,
    /// Total amount to be charged. If a service fee is included, then this amount should include the service fee.
    #[serde(rename = "totalAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<f64>,
    /// Indicates whether the payout should be bundled into a single transaction or processed separately. If set to `true`, each bill will be processed as a separate payout. If `false` or not provided, then multiple bills will be paid with a single payout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unbundled: Option<bool>,
}
