pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OcrVendorAdditionalData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web: Option<String>,
}

impl OcrVendorAdditionalData {
    pub fn builder() -> OcrVendorAdditionalDataBuilder {
        <OcrVendorAdditionalDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OcrVendorAdditionalDataBuilder {
    web: Option<String>,
}

impl OcrVendorAdditionalDataBuilder {
    pub fn web(mut self, value: impl Into<String>) -> Self {
        self.web = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OcrVendorAdditionalData`].
    pub fn build(self) -> Result<OcrVendorAdditionalData, BuildError> {
        Ok(OcrVendorAdditionalData { web: self.web })
    }
}
