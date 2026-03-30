pub use crate::prelude::*;

/// Query parameters for GetUser
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetUserQueryRequest {
    /// The entrypoint identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry: Option<String>,
    /// Entry level: 0 - partner, 2 - paypoint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<i64>,
}

impl GetUserQueryRequest {
    pub fn builder() -> GetUserQueryRequestBuilder {
        <GetUserQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetUserQueryRequestBuilder {
    entry: Option<String>,
    level: Option<i64>,
}

impl GetUserQueryRequestBuilder {
    pub fn entry(mut self, value: impl Into<String>) -> Self {
        self.entry = Some(value.into());
        self
    }

    pub fn level(mut self, value: i64) -> Self {
        self.level = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetUserQueryRequest`].
    pub fn build(self) -> Result<GetUserQueryRequest, BuildError> {
        Ok(GetUserQueryRequest {
            entry: self.entry,
            level: self.level,
        })
    }
}
