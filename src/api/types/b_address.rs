pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BAddress {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baddress: Option<LinkData>,
    #[serde(rename = "baddress1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baddress_1: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcity: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcountry: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bstate: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bzip: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maddress: Option<LinkData>,
    #[serde(rename = "maddress1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maddress_1: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcity: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcountry: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mstate: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mzip: Option<LinkData>,
}
