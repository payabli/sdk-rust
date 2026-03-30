pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AchAbsorbSection {
    #[serde(rename = "multiTier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_tier: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiers: Option<Vec<AchTypesTiers>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<Visible>,
}

impl AchAbsorbSection {
    pub fn builder() -> AchAbsorbSectionBuilder {
        <AchAbsorbSectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AchAbsorbSectionBuilder {
    multi_tier: Option<bool>,
    tiers: Option<Vec<AchTypesTiers>>,
    visible: Option<Visible>,
}

impl AchAbsorbSectionBuilder {
    pub fn multi_tier(mut self, value: bool) -> Self {
        self.multi_tier = Some(value);
        self
    }

    pub fn tiers(mut self, value: Vec<AchTypesTiers>) -> Self {
        self.tiers = Some(value);
        self
    }

    pub fn visible(mut self, value: Visible) -> Self {
        self.visible = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AchAbsorbSection`].
    pub fn build(self) -> Result<AchAbsorbSection, BuildError> {
        Ok(AchAbsorbSection {
            multi_tier: self.multi_tier,
            tiers: self.tiers,
            visible: self.visible,
        })
    }
}
