pub use crate::prelude::*;

/// Controls which services will be enabled for the merchant.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Services {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<AchSetup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CardSetup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub odp: Option<OdpSetup>,
}

impl Services {
    pub fn builder() -> ServicesBuilder {
        <ServicesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ServicesBuilder {
    ach: Option<AchSetup>,
    card: Option<CardSetup>,
    odp: Option<OdpSetup>,
}

impl ServicesBuilder {
    pub fn ach(mut self, value: AchSetup) -> Self {
        self.ach = Some(value);
        self
    }

    pub fn card(mut self, value: CardSetup) -> Self {
        self.card = Some(value);
        self
    }

    pub fn odp(mut self, value: OdpSetup) -> Self {
        self.odp = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Services`].
    pub fn build(self) -> Result<Services, BuildError> {
        Ok(Services {
            ach: self.ach,
            card: self.card,
            odp: self.odp,
        })
    }
}
