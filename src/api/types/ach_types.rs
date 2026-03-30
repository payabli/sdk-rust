pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AchTypes {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccd: Option<BasicTemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ppd: Option<BasicTemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web: Option<BasicTemplateElement>,
}

impl AchTypes {
    pub fn builder() -> AchTypesBuilder {
        <AchTypesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AchTypesBuilder {
    ccd: Option<BasicTemplateElement>,
    ppd: Option<BasicTemplateElement>,
    web: Option<BasicTemplateElement>,
}

impl AchTypesBuilder {
    pub fn ccd(mut self, value: BasicTemplateElement) -> Self {
        self.ccd = Some(value);
        self
    }

    pub fn ppd(mut self, value: BasicTemplateElement) -> Self {
        self.ppd = Some(value);
        self
    }

    pub fn web(mut self, value: BasicTemplateElement) -> Self {
        self.web = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AchTypes`].
    pub fn build(self) -> Result<AchTypes, BuildError> {
        Ok(AchTypes {
            ccd: self.ccd,
            ppd: self.ppd,
            web: self.web,
        })
    }
}
