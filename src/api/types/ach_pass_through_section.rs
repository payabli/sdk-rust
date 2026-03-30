pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AchPassThroughSection {
    #[serde(rename = "multiTier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_tier: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiers: Option<Vec<AchTypesPass>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<Visible>,
}

impl AchPassThroughSection {
    pub fn builder() -> AchPassThroughSectionBuilder {
        <AchPassThroughSectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AchPassThroughSectionBuilder {
    multi_tier: Option<bool>,
    tiers: Option<Vec<AchTypesPass>>,
    visible: Option<Visible>,
}

impl AchPassThroughSectionBuilder {
    pub fn multi_tier(mut self, value: bool) -> Self {
        self.multi_tier = Some(value);
        self
    }

    pub fn tiers(mut self, value: Vec<AchTypesPass>) -> Self {
        self.tiers = Some(value);
        self
    }

    pub fn visible(mut self, value: Visible) -> Self {
        self.visible = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AchPassThroughSection`].
    pub fn build(self) -> Result<AchPassThroughSection, BuildError> {
        Ok(AchPassThroughSection {
            multi_tier: self.multi_tier,
            tiers: self.tiers,
            visible: self.visible,
        })
    }
}
