pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PSection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avgmonthly: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binperson: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binphone: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binweb: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bsummary: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highticketamt: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcc: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticketamt: Option<LinkData>,
    #[serde(rename = "whenCharged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when_charged: Option<LinkData>,
    #[serde(rename = "whenDelivered")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when_delivered: Option<LinkData>,
    #[serde(rename = "whenProvided")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when_provided: Option<LinkData>,
    #[serde(rename = "whenRefunded")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when_refunded: Option<LinkData>,
}
