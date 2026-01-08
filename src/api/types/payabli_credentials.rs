pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PayabliCredentials {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "cfeeFix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfee_fix: Option<f64>,
    #[serde(rename = "cfeeFloat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfee_float: Option<f64>,
    #[serde(rename = "cfeeMax")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfee_max: Option<f64>,
    #[serde(rename = "cfeeMin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfee_min: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxticket: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minticket: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<i64>,
    #[serde(rename = "referenceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
}
