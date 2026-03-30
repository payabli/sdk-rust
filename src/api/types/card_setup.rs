pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CardSetup {
    /// Determines whether American Express is accepted.
    #[serde(rename = "acceptAmex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_amex: Option<bool>,
    /// Determines whether Discover is accepted.
    #[serde(rename = "acceptDiscover")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_discover: Option<bool>,
    /// Determines whether Mastercard is accepted.
    #[serde(rename = "acceptMastercard")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_mastercard: Option<bool>,
    /// Determines whether Visa is accepted.
    #[serde(rename = "acceptVisa")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_visa: Option<bool>,
}

impl CardSetup {
    pub fn builder() -> CardSetupBuilder {
        <CardSetupBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CardSetupBuilder {
    accept_amex: Option<bool>,
    accept_discover: Option<bool>,
    accept_mastercard: Option<bool>,
    accept_visa: Option<bool>,
}

impl CardSetupBuilder {
    pub fn accept_amex(mut self, value: bool) -> Self {
        self.accept_amex = Some(value);
        self
    }

    pub fn accept_discover(mut self, value: bool) -> Self {
        self.accept_discover = Some(value);
        self
    }

    pub fn accept_mastercard(mut self, value: bool) -> Self {
        self.accept_mastercard = Some(value);
        self
    }

    pub fn accept_visa(mut self, value: bool) -> Self {
        self.accept_visa = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CardSetup`].
    pub fn build(self) -> Result<CardSetup, BuildError> {
        Ok(CardSetup {
            accept_amex: self.accept_amex,
            accept_discover: self.accept_discover,
            accept_mastercard: self.accept_mastercard,
            accept_visa: self.accept_visa,
        })
    }
}
