pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ApplicationDataPayInServices {
    pub ach: ApplicationDataPayInServicesAch,
    pub card: ApplicationDataPayInServicesCard,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub odp: Option<OdpSetup>,
}
