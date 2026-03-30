pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConfigureGooglePaypointApiResponse {
    #[serde(rename = "isSuccess")]
    #[serde(default)]
    pub is_success: IsSuccess,
    #[serde(rename = "pageIdentifier")]
    #[serde(default)]
    pub page_identifier: PageIdentifier,
    #[serde(rename = "responseCode")]
    #[serde(default)]
    pub response_code: Responsecode,
    #[serde(rename = "responseData")]
    #[serde(default)]
    pub response_data: GooglePayPaypointRegistrationData,
    #[serde(rename = "responseText")]
    #[serde(default)]
    pub response_text: ResponseText,
    /// Field not in use on this endpoint
    #[serde(rename = "roomId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_id: Option<i64>,
}

impl ConfigureGooglePaypointApiResponse {
    pub fn builder() -> ConfigureGooglePaypointApiResponseBuilder {
        <ConfigureGooglePaypointApiResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConfigureGooglePaypointApiResponseBuilder {
    is_success: Option<IsSuccess>,
    page_identifier: Option<PageIdentifier>,
    response_code: Option<Responsecode>,
    response_data: Option<GooglePayPaypointRegistrationData>,
    response_text: Option<ResponseText>,
    room_id: Option<i64>,
}

impl ConfigureGooglePaypointApiResponseBuilder {
    pub fn is_success(mut self, value: IsSuccess) -> Self {
        self.is_success = Some(value);
        self
    }

    pub fn page_identifier(mut self, value: PageIdentifier) -> Self {
        self.page_identifier = Some(value);
        self
    }

    pub fn response_code(mut self, value: Responsecode) -> Self {
        self.response_code = Some(value);
        self
    }

    pub fn response_data(mut self, value: GooglePayPaypointRegistrationData) -> Self {
        self.response_data = Some(value);
        self
    }

    pub fn response_text(mut self, value: ResponseText) -> Self {
        self.response_text = Some(value);
        self
    }

    pub fn room_id(mut self, value: i64) -> Self {
        self.room_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConfigureGooglePaypointApiResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`is_success`](ConfigureGooglePaypointApiResponseBuilder::is_success)
    /// - [`page_identifier`](ConfigureGooglePaypointApiResponseBuilder::page_identifier)
    /// - [`response_code`](ConfigureGooglePaypointApiResponseBuilder::response_code)
    /// - [`response_data`](ConfigureGooglePaypointApiResponseBuilder::response_data)
    /// - [`response_text`](ConfigureGooglePaypointApiResponseBuilder::response_text)
    pub fn build(self) -> Result<ConfigureGooglePaypointApiResponse, BuildError> {
        Ok(ConfigureGooglePaypointApiResponse {
            is_success: self
                .is_success
                .ok_or_else(|| BuildError::missing_field("is_success"))?,
            page_identifier: self
                .page_identifier
                .ok_or_else(|| BuildError::missing_field("page_identifier"))?,
            response_code: self
                .response_code
                .ok_or_else(|| BuildError::missing_field("response_code"))?,
            response_data: self
                .response_data
                .ok_or_else(|| BuildError::missing_field("response_data"))?,
            response_text: self
                .response_text
                .ok_or_else(|| BuildError::missing_field("response_text"))?,
            room_id: self.room_id,
        })
    }
}
