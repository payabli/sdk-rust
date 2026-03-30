pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ApplicationDataPayInServicesCard {
    #[serde(flatten)]
    pub card_setup_fields: CardSetup,
}

impl ApplicationDataPayInServicesCard {
    pub fn builder() -> ApplicationDataPayInServicesCardBuilder {
        <ApplicationDataPayInServicesCardBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApplicationDataPayInServicesCardBuilder {
    card_setup_fields: Option<CardSetup>,
}

impl ApplicationDataPayInServicesCardBuilder {
    pub fn card_setup_fields(mut self, value: CardSetup) -> Self {
        self.card_setup_fields = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ApplicationDataPayInServicesCard`].
    /// This method will fail if any of the following fields are not set:
    /// - [`card_setup_fields`](ApplicationDataPayInServicesCardBuilder::card_setup_fields)
    pub fn build(self) -> Result<ApplicationDataPayInServicesCard, BuildError> {
        Ok(ApplicationDataPayInServicesCard {
            card_setup_fields: self
                .card_setup_fields
                .ok_or_else(|| BuildError::missing_field("card_setup_fields"))?,
        })
    }
}
