pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PayoutGatewayConnector {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<String>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<i64>,
    #[serde(rename = "Bank")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<String>,
    #[serde(rename = "Descriptor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptor: Option<String>,
    #[serde(rename = "gatewayID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_id: Option<i64>,
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "EnableACHValidation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_ach_validation: Option<bool>,
    #[serde(rename = "TestMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_mode: Option<bool>,
}
