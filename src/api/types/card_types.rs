pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CardTypes {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amex: Option<BasicTemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discover: Option<BasicTemplateElement>,
    #[serde(rename = "masterCard")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_card: Option<BasicTemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visa: Option<BasicTemplateElement>,
}

impl CardTypes {
    pub fn builder() -> CardTypesBuilder {
        <CardTypesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CardTypesBuilder {
    amex: Option<BasicTemplateElement>,
    discover: Option<BasicTemplateElement>,
    master_card: Option<BasicTemplateElement>,
    visa: Option<BasicTemplateElement>,
}

impl CardTypesBuilder {
    pub fn amex(mut self, value: BasicTemplateElement) -> Self {
        self.amex = Some(value);
        self
    }

    pub fn discover(mut self, value: BasicTemplateElement) -> Self {
        self.discover = Some(value);
        self
    }

    pub fn master_card(mut self, value: BasicTemplateElement) -> Self {
        self.master_card = Some(value);
        self
    }

    pub fn visa(mut self, value: BasicTemplateElement) -> Self {
        self.visa = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CardTypes`].
    pub fn build(self) -> Result<CardTypes, BuildError> {
        Ok(CardTypes {
            amex: self.amex,
            discover: self.discover,
            master_card: self.master_card,
            visa: self.visa,
        })
    }
}
