pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SummaryOrg {
    #[serde(rename = "amountSubs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_subs: Option<f64>,
    #[serde(rename = "amountTx")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_tx: Option<f64>,
    #[serde(rename = "childOrgs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_orgs: Option<i64>,
    #[serde(rename = "childPaypoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_paypoints: Option<i64>,
    #[serde(rename = "countSubs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_subs: Option<i64>,
    #[serde(rename = "countTx")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_tx: Option<i64>,
}
