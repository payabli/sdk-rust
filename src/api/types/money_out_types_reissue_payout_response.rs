pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReissuePayoutResponse {
    #[serde(rename = "isSuccess")]
    #[serde(default)]
    pub is_success: IsSuccess,
    #[serde(rename = "responseCode")]
    #[serde(default)]
    pub response_code: Responsecode,
    #[serde(rename = "responseText")]
    #[serde(default)]
    pub response_text: ResponseText,
    #[serde(rename = "responseData")]
    #[serde(default)]
    pub response_data: ReissuePayoutResponseData,
}

impl ReissuePayoutResponse {
    pub fn builder() -> ReissuePayoutResponseBuilder {
        <ReissuePayoutResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReissuePayoutResponseBuilder {
    is_success: Option<IsSuccess>,
    response_code: Option<Responsecode>,
    response_text: Option<ResponseText>,
    response_data: Option<ReissuePayoutResponseData>,
}

impl ReissuePayoutResponseBuilder {
    pub fn is_success(mut self, value: IsSuccess) -> Self {
        self.is_success = Some(value);
        self
    }

    pub fn response_code(mut self, value: Responsecode) -> Self {
        self.response_code = Some(value);
        self
    }

    pub fn response_text(mut self, value: ResponseText) -> Self {
        self.response_text = Some(value);
        self
    }

    pub fn response_data(mut self, value: ReissuePayoutResponseData) -> Self {
        self.response_data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReissuePayoutResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`is_success`](ReissuePayoutResponseBuilder::is_success)
    /// - [`response_code`](ReissuePayoutResponseBuilder::response_code)
    /// - [`response_text`](ReissuePayoutResponseBuilder::response_text)
    /// - [`response_data`](ReissuePayoutResponseBuilder::response_data)
    pub fn build(self) -> Result<ReissuePayoutResponse, BuildError> {
        Ok(ReissuePayoutResponse {
            is_success: self
                .is_success
                .ok_or_else(|| BuildError::missing_field("is_success"))?,
            response_code: self
                .response_code
                .ok_or_else(|| BuildError::missing_field("response_code"))?,
            response_text: self
                .response_text
                .ok_or_else(|| BuildError::missing_field("response_text"))?,
            response_data: self
                .response_data
                .ok_or_else(|| BuildError::missing_field("response_data"))?,
        })
    }
}
