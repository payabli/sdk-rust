pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CardType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amex: Option<TierItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discover: Option<TierItem>,
    #[serde(rename = "masterCard")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_card: Option<TierItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visa: Option<TierItem>,
}

impl CardType {
    pub fn builder() -> CardTypeBuilder {
        <CardTypeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CardTypeBuilder {
    amex: Option<TierItem>,
    discover: Option<TierItem>,
    master_card: Option<TierItem>,
    visa: Option<TierItem>,
}

impl CardTypeBuilder {
    pub fn amex(mut self, value: TierItem) -> Self {
        self.amex = Some(value);
        self
    }

    pub fn discover(mut self, value: TierItem) -> Self {
        self.discover = Some(value);
        self
    }

    pub fn master_card(mut self, value: TierItem) -> Self {
        self.master_card = Some(value);
        self
    }

    pub fn visa(mut self, value: TierItem) -> Self {
        self.visa = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CardType`].
    pub fn build(self) -> Result<CardType, BuildError> {
        Ok(CardType {
            amex: self.amex,
            discover: self.discover,
            master_card: self.master_card,
            visa: self.visa,
        })
    }
}
