pub use crate::prelude::*;

/// Details about the Apple Pay service status.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ApplePayStatusData {
    /// Any error message related to Apple Pay's activation status.
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ApplePayMetadata>,
}

impl ApplePayStatusData {
    pub fn builder() -> ApplePayStatusDataBuilder {
        <ApplePayStatusDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApplePayStatusDataBuilder {
    error_message: Option<String>,
    metadata: Option<ApplePayMetadata>,
}

impl ApplePayStatusDataBuilder {
    pub fn error_message(mut self, value: impl Into<String>) -> Self {
        self.error_message = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: ApplePayMetadata) -> Self {
        self.metadata = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ApplePayStatusData`].
    pub fn build(self) -> Result<ApplePayStatusData, BuildError> {
        Ok(ApplePayStatusData {
            error_message: self.error_message,
            metadata: self.metadata,
        })
    }
}
