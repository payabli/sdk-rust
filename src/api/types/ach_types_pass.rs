pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AchTypesPass {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccd: Option<TierItemPass>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ppd: Option<TierItemPass>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web: Option<TierItemPass>,
}

impl AchTypesPass {
    pub fn builder() -> AchTypesPassBuilder {
        <AchTypesPassBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AchTypesPassBuilder {
    ccd: Option<TierItemPass>,
    ppd: Option<TierItemPass>,
    web: Option<TierItemPass>,
}

impl AchTypesPassBuilder {
    pub fn ccd(mut self, value: TierItemPass) -> Self {
        self.ccd = Some(value);
        self
    }

    pub fn ppd(mut self, value: TierItemPass) -> Self {
        self.ppd = Some(value);
        self
    }

    pub fn web(mut self, value: TierItemPass) -> Self {
        self.web = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AchTypesPass`].
    pub fn build(self) -> Result<AchTypesPass, BuildError> {
        Ok(AchTypesPass {
            ccd: self.ccd,
            ppd: self.ppd,
            web: self.web,
        })
    }
}
