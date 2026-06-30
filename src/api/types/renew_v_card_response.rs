pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RenewVCardResponse {
    #[serde(rename = "responseText")]
    #[serde(default)]
    pub response_text: ResponseText,
    #[serde(rename = "isSuccess")]
    #[serde(default)]
    pub is_success: IsSuccess,
    #[serde(rename = "responseData")]
    #[serde(default)]
    pub response_data: RenewVCardResponseData,
}

impl RenewVCardResponse {
    pub fn builder() -> RenewVCardResponseBuilder {
        <RenewVCardResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RenewVCardResponseBuilder {
    response_text: Option<ResponseText>,
    is_success: Option<IsSuccess>,
    response_data: Option<RenewVCardResponseData>,
}

impl RenewVCardResponseBuilder {
    pub fn response_text(mut self, value: ResponseText) -> Self {
        self.response_text = Some(value);
        self
    }

    pub fn is_success(mut self, value: IsSuccess) -> Self {
        self.is_success = Some(value);
        self
    }

    pub fn response_data(mut self, value: RenewVCardResponseData) -> Self {
        self.response_data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RenewVCardResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`response_text`](RenewVCardResponseBuilder::response_text)
    /// - [`is_success`](RenewVCardResponseBuilder::is_success)
    /// - [`response_data`](RenewVCardResponseBuilder::response_data)
    pub fn build(self) -> Result<RenewVCardResponse, BuildError> {
        Ok(RenewVCardResponse {
            response_text: self
                .response_text
                .ok_or_else(|| BuildError::missing_field("response_text"))?,
            is_success: self
                .is_success
                .ok_or_else(|| BuildError::missing_field("is_success"))?,
            response_data: self
                .response_data
                .ok_or_else(|| BuildError::missing_field("response_data"))?,
        })
    }
}
