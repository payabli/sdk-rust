pub use crate::prelude::*;

/// Hypermedia links for a person resource.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PersonLinks {
    /// Relative path to the person resource.
    #[serde(rename = "self")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    /// Relative path to the linked business, present when the person is linked to a business.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business: Option<String>,
}

impl PersonLinks {
    pub fn builder() -> PersonLinksBuilder {
        <PersonLinksBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PersonLinksBuilder {
    self_: Option<String>,
    business: Option<String>,
}

impl PersonLinksBuilder {
    pub fn self_(mut self, value: impl Into<String>) -> Self {
        self.self_ = Some(value.into());
        self
    }

    pub fn business(mut self, value: impl Into<String>) -> Self {
        self.business = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PersonLinks`].
    pub fn build(self) -> Result<PersonLinks, BuildError> {
        Ok(PersonLinks {
            self_: self.self_,
            business: self.business,
        })
    }
}
