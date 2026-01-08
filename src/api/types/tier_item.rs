pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TierItem {
    #[serde(rename = "amountxAuth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amountx_auth: Option<f64>,
    #[serde(rename = "highPayRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_pay_range: Option<f64>,
    #[serde(rename = "lowPayRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low_pay_range: Option<f64>,
    #[serde(rename = "percentxAuth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentx_auth: Option<f64>,
}
