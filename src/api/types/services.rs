pub use crate::prelude::*;

/// Controls which services will be enabled for the merchant.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Services {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<AchSetup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CardSetup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub odp: Option<OdpSetup>,
}
