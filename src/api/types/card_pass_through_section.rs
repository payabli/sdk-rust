pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CardPassThroughSection {
    #[serde(rename = "multiTier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_tier: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiers: Option<Vec<CardTypePass>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<Visible>,
}

impl CardPassThroughSection {
    pub fn builder() -> CardPassThroughSectionBuilder {
        <CardPassThroughSectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CardPassThroughSectionBuilder {
    multi_tier: Option<bool>,
    tiers: Option<Vec<CardTypePass>>,
    visible: Option<Visible>,
}

impl CardPassThroughSectionBuilder {
    pub fn multi_tier(mut self, value: bool) -> Self {
        self.multi_tier = Some(value);
        self
    }

    pub fn tiers(mut self, value: Vec<CardTypePass>) -> Self {
        self.tiers = Some(value);
        self
    }

    pub fn visible(mut self, value: Visible) -> Self {
        self.visible = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CardPassThroughSection`].
    pub fn build(self) -> Result<CardPassThroughSection, BuildError> {
        Ok(CardPassThroughSection {
            multi_tier: self.multi_tier,
            tiers: self.tiers,
            visible: self.visible,
        })
    }
}
