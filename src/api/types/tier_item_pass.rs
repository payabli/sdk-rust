pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TierItemPass {
    #[serde(rename = "amountFeeone-time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_feeone_time: Option<f64>,
    #[serde(rename = "amountFeeRecurring")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_fee_recurring: Option<f64>,
    #[serde(rename = "highPayRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_pay_range: Option<f64>,
    #[serde(rename = "lowPayRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low_pay_range: Option<f64>,
    #[serde(rename = "percentFeeone-time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_feeone_time: Option<f64>,
    #[serde(rename = "percentFeeRecurring")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_fee_recurring: Option<f64>,
}
