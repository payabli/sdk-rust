pub use crate::prelude::*;

/// The PaymentDetail object for microdeposit (MakeCredit) transactions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaymentDetailCredit {
    /// Currency code ISO-4217. If not code is provided the currency in the paypoint setting is taken. Default is **USD**
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Service fee to be deducted from the total amount. This amount must be a number, percentages aren't accepted. If you are using a percentage-based fee schedule, you must calculate the value manually.
    #[serde(rename = "serviceFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_fee: Option<f64>,
    /// Total amount to be charged. If a service fee is provided, then this amount should include the service fee.
    #[serde(rename = "totalAmount")]
    pub total_amount: f64,
}
