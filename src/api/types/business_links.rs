pub use crate::prelude::*;

/// Hypermedia links for a business resource.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BusinessLinks {
    /// Relative path to the business resource.
    #[serde(rename = "self")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl BusinessLinks {
    pub fn builder() -> BusinessLinksBuilder {
        <BusinessLinksBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BusinessLinksBuilder {
    self_: Option<String>,
}

impl BusinessLinksBuilder {
    pub fn self_(mut self, value: impl Into<String>) -> Self {
        self.self_ = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BusinessLinks`].
    pub fn build(self) -> Result<BusinessLinks, BuildError> {
        Ok(BusinessLinks { self_: self.self_ })
    }
}
