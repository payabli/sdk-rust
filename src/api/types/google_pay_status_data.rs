pub use crate::prelude::*;

/// Details about the Google Pay service status.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GooglePayStatusData {
    /// Any error message related to Google Pay's activation status.
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<GooglePayMetadata>,
}

impl GooglePayStatusData {
    pub fn builder() -> GooglePayStatusDataBuilder {
        <GooglePayStatusDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GooglePayStatusDataBuilder {
    error_message: Option<String>,
    metadata: Option<GooglePayMetadata>,
}

impl GooglePayStatusDataBuilder {
    pub fn error_message(mut self, value: impl Into<String>) -> Self {
        self.error_message = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: GooglePayMetadata) -> Self {
        self.metadata = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GooglePayStatusData`].
    pub fn build(self) -> Result<GooglePayStatusData, BuildError> {
        Ok(GooglePayStatusData {
            error_message: self.error_message,
            metadata: self.metadata,
        })
    }
}
