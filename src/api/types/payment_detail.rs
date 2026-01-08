pub use crate::prelude::*;

/// Details about the payment.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaymentDetail {
    /// Array of payment categories/line items describing the amount to be paid.
    /// **Note**: These categories are for information only and aren't validated against the total amount provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<PaymentCategories>>,
    /// Object containing image of paper check.
    #[serde(rename = "checkImage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_image: Option<HashMap<String, serde_json::Value>>,
    /// A check number to be used in the ach transaction. **Required** for payment method = 'check'.
    #[serde(rename = "checkNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_number: Option<String>,
    /// The currency for the transaction, `USD` or `CAD`. If your paypoint is configured for CAD, you must send the `CAD` value in this field, otherwise it defaults to USD, which will cause the transaction to fail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Service fee to be deducted from the total amount. This amount must be a number, percentages aren't accepted. If you are using a percentage-based fee schedule, you must calculate the value manually.
    #[serde(rename = "serviceFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_fee: Option<f64>,
    /// Split funding instructions for the transaction. See [Split a Transaction](/developers/developer-guides/money-in-split-funding) for more.
    #[serde(rename = "splitFunding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_funding: Option<SplitFunding>,
    /// Total amount to be charged. If a service fee is sent, then this amount should include the service fee."
    #[serde(rename = "totalAmount")]
    pub total_amount: f64,
}
