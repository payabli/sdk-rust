pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BuilderData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<SSection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<ASection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banking: Option<DSection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business: Option<BSection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owners: Option<OSection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing: Option<PSection>,
}
