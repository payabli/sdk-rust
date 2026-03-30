pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BSection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<BAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<BDetails>,
}

impl BSection {
    pub fn builder() -> BSectionBuilder {
        <BSectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BSectionBuilder {
    address: Option<BAddress>,
    details: Option<BDetails>,
}

impl BSectionBuilder {
    pub fn address(mut self, value: BAddress) -> Self {
        self.address = Some(value);
        self
    }

    pub fn details(mut self, value: BDetails) -> Self {
        self.details = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BSection`].
    pub fn build(self) -> Result<BSection, BuildError> {
        Ok(BSection {
            address: self.address,
            details: self.details,
        })
    }
}
