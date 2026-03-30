pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AchTypesTiers {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccd: Option<TierItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ppd: Option<TierItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web: Option<TierItem>,
}

impl AchTypesTiers {
    pub fn builder() -> AchTypesTiersBuilder {
        <AchTypesTiersBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AchTypesTiersBuilder {
    ccd: Option<TierItem>,
    ppd: Option<TierItem>,
    web: Option<TierItem>,
}

impl AchTypesTiersBuilder {
    pub fn ccd(mut self, value: TierItem) -> Self {
        self.ccd = Some(value);
        self
    }

    pub fn ppd(mut self, value: TierItem) -> Self {
        self.ppd = Some(value);
        self
    }

    pub fn web(mut self, value: TierItem) -> Self {
        self.web = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AchTypesTiers`].
    pub fn build(self) -> Result<AchTypesTiers, BuildError> {
        Ok(AchTypesTiers {
            ccd: self.ccd,
            ppd: self.ppd,
            web: self.web,
        })
    }
}
