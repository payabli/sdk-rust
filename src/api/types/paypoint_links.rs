pub use crate::prelude::*;

/// Hypermedia links for a paypoint resource.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaypointLinks {
    /// Relative path to the paypoint resource.
    #[serde(rename = "self")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    /// Relative path to the business the paypoint belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business: Option<String>,
}

impl PaypointLinks {
    pub fn builder() -> PaypointLinksBuilder {
        <PaypointLinksBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaypointLinksBuilder {
    self_: Option<String>,
    business: Option<String>,
}

impl PaypointLinksBuilder {
    pub fn self_(mut self, value: impl Into<String>) -> Self {
        self.self_ = Some(value.into());
        self
    }

    pub fn business(mut self, value: impl Into<String>) -> Self {
        self.business = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PaypointLinks`].
    pub fn build(self) -> Result<PaypointLinks, BuildError> {
        Ok(PaypointLinks {
            self_: self.self_,
            business: self.business,
        })
    }
}
