pub use crate::prelude::*;

/// Hypermedia links for an application resource.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ApplicationLinks {
    /// Relative path to the application resource.
    #[serde(rename = "self")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    /// Relative path to the business the application boards.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business: Option<String>,
}

impl ApplicationLinks {
    pub fn builder() -> ApplicationLinksBuilder {
        <ApplicationLinksBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApplicationLinksBuilder {
    self_: Option<String>,
    business: Option<String>,
}

impl ApplicationLinksBuilder {
    pub fn self_(mut self, value: impl Into<String>) -> Self {
        self.self_ = Some(value.into());
        self
    }

    pub fn business(mut self, value: impl Into<String>) -> Self {
        self.business = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ApplicationLinks`].
    pub fn build(self) -> Result<ApplicationLinks, BuildError> {
        Ok(ApplicationLinks {
            self_: self.self_,
            business: self.business,
        })
    }
}
