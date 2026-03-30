pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CardTypePass {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amex: Option<TierItemPass>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discover: Option<TierItemPass>,
    #[serde(rename = "masterCard")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_card: Option<TierItemPass>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visa: Option<TierItemPass>,
}

impl CardTypePass {
    pub fn builder() -> CardTypePassBuilder {
        <CardTypePassBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CardTypePassBuilder {
    amex: Option<TierItemPass>,
    discover: Option<TierItemPass>,
    master_card: Option<TierItemPass>,
    visa: Option<TierItemPass>,
}

impl CardTypePassBuilder {
    pub fn amex(mut self, value: TierItemPass) -> Self {
        self.amex = Some(value);
        self
    }

    pub fn discover(mut self, value: TierItemPass) -> Self {
        self.discover = Some(value);
        self
    }

    pub fn master_card(mut self, value: TierItemPass) -> Self {
        self.master_card = Some(value);
        self
    }

    pub fn visa(mut self, value: TierItemPass) -> Self {
        self.visa = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CardTypePass`].
    pub fn build(self) -> Result<CardTypePass, BuildError> {
        Ok(CardTypePass {
            amex: self.amex,
            discover: self.discover,
            master_card: self.master_card,
            visa: self.visa,
        })
    }
}
