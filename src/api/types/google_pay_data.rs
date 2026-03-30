pub use crate::prelude::*;

/// Details about the status of the Google Pay service.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GooglePayData {
    /// This object is only returned when the domain verification check fails. If a domain has failed validation, this object contains information about the failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<GooglePayStatusData>,
    /// When `true`, Google Pay is enabled.
    #[serde(rename = "isEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<IsEnabled>,
}

impl GooglePayData {
    pub fn builder() -> GooglePayDataBuilder {
        <GooglePayDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GooglePayDataBuilder {
    data: Option<GooglePayStatusData>,
    is_enabled: Option<IsEnabled>,
}

impl GooglePayDataBuilder {
    pub fn data(mut self, value: GooglePayStatusData) -> Self {
        self.data = Some(value);
        self
    }

    pub fn is_enabled(mut self, value: IsEnabled) -> Self {
        self.is_enabled = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GooglePayData`].
    pub fn build(self) -> Result<GooglePayData, BuildError> {
        Ok(GooglePayData {
            data: self.data,
            is_enabled: self.is_enabled,
        })
    }
}
