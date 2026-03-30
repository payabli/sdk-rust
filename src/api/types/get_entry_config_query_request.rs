pub use crate::prelude::*;

/// Query parameters for getEntryConfig
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetEntryConfigQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entrypages: Option<String>,
}

impl GetEntryConfigQueryRequest {
    pub fn builder() -> GetEntryConfigQueryRequestBuilder {
        <GetEntryConfigQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetEntryConfigQueryRequestBuilder {
    entrypages: Option<String>,
}

impl GetEntryConfigQueryRequestBuilder {
    pub fn entrypages(mut self, value: impl Into<String>) -> Self {
        self.entrypages = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetEntryConfigQueryRequest`].
    pub fn build(self) -> Result<GetEntryConfigQueryRequest, BuildError> {
        Ok(GetEntryConfigQueryRequest {
            entrypages: self.entrypages,
        })
    }
}
