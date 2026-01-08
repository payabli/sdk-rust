pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ApplicationDataPayInServicesCard {
    #[serde(flatten)]
    pub card_setup_fields: CardSetup,
}
