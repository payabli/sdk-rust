pub use crate::prelude::*;

/// Hypermedia links for a business-person link.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BusinessPersonLinks {
    /// Relative path to the business-person link.
    #[serde(rename = "self")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    /// Relative path to the linked business.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business: Option<String>,
    /// Relative path to the linked person.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person: Option<String>,
}

impl BusinessPersonLinks {
    pub fn builder() -> BusinessPersonLinksBuilder {
        <BusinessPersonLinksBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BusinessPersonLinksBuilder {
    self_: Option<String>,
    business: Option<String>,
    person: Option<String>,
}

impl BusinessPersonLinksBuilder {
    pub fn self_(mut self, value: impl Into<String>) -> Self {
        self.self_ = Some(value.into());
        self
    }

    pub fn business(mut self, value: impl Into<String>) -> Self {
        self.business = Some(value.into());
        self
    }

    pub fn person(mut self, value: impl Into<String>) -> Self {
        self.person = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BusinessPersonLinks`].
    pub fn build(self) -> Result<BusinessPersonLinks, BuildError> {
        Ok(BusinessPersonLinks {
            self_: self.self_,
            business: self.business,
            person: self.person,
        })
    }
}
