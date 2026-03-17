pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub btype: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbaname: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ein: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub faxnumber: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legalname: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub licstate: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phonenumber: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startdate: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxfillname: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<LinkData>,
}
