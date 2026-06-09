pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReverseResponse {
    #[serde(rename = "responseCode")]
    #[serde(default)]
    pub response_code: Responsecode,
    #[serde(rename = "pageIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_identifier: Option<PageIdentifier>,
    #[serde(rename = "roomId")]
    #[serde(default)]
    pub room_id: i64,
    #[serde(rename = "isSuccess")]
    #[serde(default)]
    pub is_success: IsSuccess,
    #[serde(rename = "responseText")]
    #[serde(default)]
    pub response_text: ResponseText,
    #[serde(rename = "responseData")]
    #[serde(default)]
    pub response_data: ResponseDataRefunds,
}

impl ReverseResponse {
    pub fn builder() -> ReverseResponseBuilder {
        <ReverseResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReverseResponseBuilder {
    response_code: Option<Responsecode>,
    page_identifier: Option<PageIdentifier>,
    room_id: Option<i64>,
    is_success: Option<IsSuccess>,
    response_text: Option<ResponseText>,
    response_data: Option<ResponseDataRefunds>,
}

impl ReverseResponseBuilder {
    pub fn response_code(mut self, value: Responsecode) -> Self {
        self.response_code = Some(value);
        self
    }

    pub fn page_identifier(mut self, value: PageIdentifier) -> Self {
        self.page_identifier = Some(value);
        self
    }

    pub fn room_id(mut self, value: i64) -> Self {
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

    pub fn response_data(mut self, value: ResponseDataRefunds) -> Self {
        self.response_data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReverseResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`response_code`](ReverseResponseBuilder::response_code)
    /// - [`room_id`](ReverseResponseBuilder::room_id)
    /// - [`is_success`](ReverseResponseBuilder::is_success)
    /// - [`response_text`](ReverseResponseBuilder::response_text)
    /// - [`response_data`](ReverseResponseBuilder::response_data)
    pub fn build(self) -> Result<ReverseResponse, BuildError> {
        Ok(ReverseResponse {
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
