pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ServiceCost {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<Enabled>,
    #[serde(rename = "monthlyCost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monthly_cost: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reseller: Option<bool>,
    #[serde(rename = "setupCost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_cost: Option<f64>,
    #[serde(rename = "txCost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_cost: Option<f64>,
    #[serde(rename = "txPercentCost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_percent_cost: Option<f64>,
}
