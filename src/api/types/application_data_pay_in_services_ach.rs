pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ApplicationDataPayInServicesAch {
    #[serde(flatten)]
    pub ach_setup_fields: AchSetup,
}
