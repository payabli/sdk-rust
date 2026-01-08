pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct OList {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oaddress: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocity: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocountry: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub odriverstate: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ostate: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownerdob: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownerdriver: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owneremail: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownername: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownerpercent: Option<LinkData>,
    #[serde(rename = "ownerphone1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownerphone_1: Option<LinkData>,
    #[serde(rename = "ownerphone2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownerphone_2: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownerssn: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownertitle: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ozip: Option<LinkData>,
}
