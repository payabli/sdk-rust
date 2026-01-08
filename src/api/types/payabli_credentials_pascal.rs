pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PayabliCredentialsPascal {
    #[serde(rename = "Service")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<i64>,
    #[serde(rename = "MinTicket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_ticket: Option<f64>,
    #[serde(rename = "MaxTicket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_ticket: Option<f64>,
    #[serde(rename = "CfeeFix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfee_fix: Option<f64>,
    #[serde(rename = "CfeeFloat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfee_float: Option<f64>,
    #[serde(rename = "CfeeMin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfee_min: Option<f64>,
    #[serde(rename = "CfeeMax")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfee_max: Option<f64>,
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "ReferenceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<i64>,
    #[serde(rename = "acceptSameDayACH")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_same_day_ach: Option<bool>,
    /// The default currency for the paypoint, either `USD` or `CAD`.
    #[serde(rename = "Currency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}
