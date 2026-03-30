pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CardLinkTypes {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amex: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discover: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mastercard: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visa: Option<LinkData>,
}

impl CardLinkTypes {
    pub fn builder() -> CardLinkTypesBuilder {
        <CardLinkTypesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CardLinkTypesBuilder {
    amex: Option<LinkData>,
    discover: Option<LinkData>,
    mastercard: Option<LinkData>,
    visa: Option<LinkData>,
}

impl CardLinkTypesBuilder {
    pub fn amex(mut self, value: LinkData) -> Self {
        self.amex = Some(value);
        self
    }

    pub fn discover(mut self, value: LinkData) -> Self {
        self.discover = Some(value);
        self
    }

    pub fn mastercard(mut self, value: LinkData) -> Self {
        self.mastercard = Some(value);
        self
    }

    pub fn visa(mut self, value: LinkData) -> Self {
        self.visa = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CardLinkTypes`].
    pub fn build(self) -> Result<CardLinkTypes, BuildError> {
        Ok(CardLinkTypes {
            amex: self.amex,
            discover: self.discover,
            mastercard: self.mastercard,
            visa: self.visa,
        })
    }
}
