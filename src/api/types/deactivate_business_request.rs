pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeactivateBusinessRequest {
    /// Optional reason for deactivation (max 500 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl DeactivateBusinessRequest {
    pub fn builder() -> DeactivateBusinessRequestBuilder {
        <DeactivateBusinessRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeactivateBusinessRequestBuilder {
    reason: Option<String>,
}

impl DeactivateBusinessRequestBuilder {
    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeactivateBusinessRequest`].
    pub fn build(self) -> Result<DeactivateBusinessRequest, BuildError> {
        Ok(DeactivateBusinessRequest {
            reason: self.reason,
        })
    }
}
