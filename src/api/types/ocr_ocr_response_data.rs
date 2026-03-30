pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OcrResponseData {
    #[serde(rename = "resultData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_data: Option<OcrResultData>,
}

impl OcrResponseData {
    pub fn builder() -> OcrResponseDataBuilder {
        <OcrResponseDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OcrResponseDataBuilder {
    result_data: Option<OcrResultData>,
}

impl OcrResponseDataBuilder {
    pub fn result_data(mut self, value: OcrResultData) -> Self {
        self.result_data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OcrResponseData`].
    pub fn build(self) -> Result<OcrResponseData, BuildError> {
        Ok(OcrResponseData {
            result_data: self.result_data,
        })
    }
}
