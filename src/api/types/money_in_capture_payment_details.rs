pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CapturePaymentDetails {
    /// Total amount to be captured, including the `serviceFee` amount. The amount can't be greater the original
    /// total amount of the transaction, and can't be more than 15% lower than the original amount.
    #[serde(rename = "totalAmount")]
    pub total_amount: f64,
    /// Service fee to capture for the transaction.
    #[serde(rename = "serviceFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_fee: Option<f64>,
}
