pub use crate::prelude::*;

/// Object containing details about cloud devices and their registration history.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CloudQueryApiResponse {
    #[serde(rename = "isSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<IsSuccess>,
    /// List of devices and history of registration.
    #[serde(rename = "responseList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_list: Option<Vec<PoiDevice>>,
    #[serde(rename = "responseText")]
    #[serde(default)]
    pub response_text: ResponseText,
}

impl CloudQueryApiResponse {
    pub fn builder() -> CloudQueryApiResponseBuilder {
        <CloudQueryApiResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CloudQueryApiResponseBuilder {
    is_success: Option<IsSuccess>,
    response_list: Option<Vec<PoiDevice>>,
    response_text: Option<ResponseText>,
}

impl CloudQueryApiResponseBuilder {
    pub fn is_success(mut self, value: IsSuccess) -> Self {
        self.is_success = Some(value);
        self
    }

    pub fn response_list(mut self, value: Vec<PoiDevice>) -> Self {
        self.response_list = Some(value);
        self
    }

    pub fn response_text(mut self, value: ResponseText) -> Self {
        self.response_text = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CloudQueryApiResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`response_text`](CloudQueryApiResponseBuilder::response_text)
    pub fn build(self) -> Result<CloudQueryApiResponse, BuildError> {
        Ok(CloudQueryApiResponse {
            is_success: self.is_success,
            response_list: self.response_list,
            response_text: self
                .response_text
                .ok_or_else(|| BuildError::missing_field("response_text"))?,
        })
    }
}
