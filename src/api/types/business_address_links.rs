pub use crate::prelude::*;

/// Hypermedia links for a business address resource.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BusinessAddressLinks {
    /// Relative path to the business address resource.
    #[serde(rename = "self")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    /// Relative path to the business that owns the address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business: Option<String>,
}

impl BusinessAddressLinks {
    pub fn builder() -> BusinessAddressLinksBuilder {
        <BusinessAddressLinksBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BusinessAddressLinksBuilder {
    self_: Option<String>,
    business: Option<String>,
}

impl BusinessAddressLinksBuilder {
    pub fn self_(mut self, value: impl Into<String>) -> Self {
        self.self_ = Some(value.into());
        self
    }

    pub fn business(mut self, value: impl Into<String>) -> Self {
        self.business = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BusinessAddressLinks`].
    pub fn build(self) -> Result<BusinessAddressLinks, BuildError> {
        Ok(BusinessAddressLinks {
            self_: self.self_,
            business: self.business,
        })
    }
}
