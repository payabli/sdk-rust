pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AuthCapturePayoutResponse {
    #[serde(rename = "responseCode")]
    #[serde(default)]
    pub response_code: Responsecode,
    #[serde(rename = "pageIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_identifier: Option<PageIdentifier>,
    #[serde(rename = "roomId")]
    #[serde(default)]
    pub room_id: RoomIdNotInUse,
    #[serde(rename = "isSuccess")]
    #[serde(default)]
    pub is_success: IsSuccess,
    #[serde(rename = "responseText")]
    #[serde(default)]
    pub response_text: ResponseText,
    #[serde(rename = "responseData")]
    #[serde(default)]
    pub response_data: AuthCapturePayoutResponseData,
}

impl AuthCapturePayoutResponse {
    pub fn builder() -> AuthCapturePayoutResponseBuilder {
        <AuthCapturePayoutResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AuthCapturePayoutResponseBuilder {
    response_code: Option<Responsecode>,
    page_identifier: Option<PageIdentifier>,
    room_id: Option<RoomIdNotInUse>,
    is_success: Option<IsSuccess>,
    response_text: Option<ResponseText>,
    response_data: Option<AuthCapturePayoutResponseData>,
}

impl AuthCapturePayoutResponseBuilder {
    pub fn response_code(mut self, value: Responsecode) -> Self {
        self.response_code = Some(value);
        self
    }

    pub fn page_identifier(mut self, value: PageIdentifier) -> Self {
        self.page_identifier = Some(value);
        self
    }

    pub fn room_id(mut self, value: RoomIdNotInUse) -> Self {
        self.room_id = Some(value);
        self
    }

    pub fn is_success(mut self, value: IsSuccess) -> Self {
        self.is_success = Some(value);
        self
    }

    pub fn response_text(mut self, value: ResponseText) -> Self {
        self.response_text = Some(value);
        self
    }

    pub fn response_data(mut self, value: AuthCapturePayoutResponseData) -> Self {
        self.response_data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AuthCapturePayoutResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`response_code`](AuthCapturePayoutResponseBuilder::response_code)
    /// - [`room_id`](AuthCapturePayoutResponseBuilder::room_id)
    /// - [`is_success`](AuthCapturePayoutResponseBuilder::is_success)
    /// - [`response_text`](AuthCapturePayoutResponseBuilder::response_text)
    /// - [`response_data`](AuthCapturePayoutResponseBuilder::response_data)
    pub fn build(self) -> Result<AuthCapturePayoutResponse, BuildError> {
        Ok(AuthCapturePayoutResponse {
            response_code: self
                .response_code
                .ok_or_else(|| BuildError::missing_field("response_code"))?,
            page_identifier: self.page_identifier,
            room_id: self
                .room_id
                .ok_or_else(|| BuildError::missing_field("room_id"))?,
            is_success: self
                .is_success
                .ok_or_else(|| BuildError::missing_field("is_success"))?,
            response_text: self
                .response_text
                .ok_or_else(|| BuildError::missing_field("response_text"))?,
            response_data: self
                .response_data
                .ok_or_else(|| BuildError::missing_field("response_data"))?,
        })
    }
}
