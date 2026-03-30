pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AchLinkTypes {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccd: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ppd: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web: Option<LinkData>,
}

impl AchLinkTypes {
    pub fn builder() -> AchLinkTypesBuilder {
        <AchLinkTypesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AchLinkTypesBuilder {
    ccd: Option<LinkData>,
    ppd: Option<LinkData>,
    web: Option<LinkData>,
}

impl AchLinkTypesBuilder {
    pub fn ccd(mut self, value: LinkData) -> Self {
        self.ccd = Some(value);
        self
    }

    pub fn ppd(mut self, value: LinkData) -> Self {
        self.ppd = Some(value);
        self
    }

    pub fn web(mut self, value: LinkData) -> Self {
        self.web = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AchLinkTypes`].
    pub fn build(self) -> Result<AchLinkTypes, BuildError> {
        Ok(AchLinkTypes {
            ccd: self.ccd,
            ppd: self.ppd,
            web: self.web,
        })
    }
}
