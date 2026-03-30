pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ApplicationDataPayInServices {
    #[serde(default)]
    pub ach: ApplicationDataPayInServicesAch,
    #[serde(default)]
    pub card: ApplicationDataPayInServicesCard,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub odp: Option<OdpSetup>,
}

impl ApplicationDataPayInServices {
    pub fn builder() -> ApplicationDataPayInServicesBuilder {
        <ApplicationDataPayInServicesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApplicationDataPayInServicesBuilder {
    ach: Option<ApplicationDataPayInServicesAch>,
    card: Option<ApplicationDataPayInServicesCard>,
    odp: Option<OdpSetup>,
}

impl ApplicationDataPayInServicesBuilder {
    pub fn ach(mut self, value: ApplicationDataPayInServicesAch) -> Self {
        self.ach = Some(value);
        self
    }

    pub fn card(mut self, value: ApplicationDataPayInServicesCard) -> Self {
        self.card = Some(value);
        self
    }

    pub fn odp(mut self, value: OdpSetup) -> Self {
        self.odp = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ApplicationDataPayInServices`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ach`](ApplicationDataPayInServicesBuilder::ach)
    /// - [`card`](ApplicationDataPayInServicesBuilder::card)
    pub fn build(self) -> Result<ApplicationDataPayInServices, BuildError> {
        Ok(ApplicationDataPayInServices {
            ach: self.ach.ok_or_else(|| BuildError::missing_field("ach"))?,
            card: self.card.ok_or_else(|| BuildError::missing_field("card"))?,
            odp: self.odp,
        })
    }
}
