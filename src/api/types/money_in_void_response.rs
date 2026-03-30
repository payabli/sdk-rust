pub use crate::prelude::*;

/// Response for MoneyIn/void endpoint
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VoidResponse {
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
    pub response_data: VoidResponseData,
}

impl VoidResponse {
    pub fn builder() -> VoidResponseBuilder {
        <VoidResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VoidResponseBuilder {
    response_code: Option<Responsecode>,
    page_identifier: Option<PageIdentifier>,
    room_id: Option<RoomIdNotInUse>,
    is_success: Option<IsSuccess>,
    response_text: Option<ResponseText>,
    response_data: Option<VoidResponseData>,
}

impl VoidResponseBuilder {
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

    pub fn response_data(mut self, value: VoidResponseData) -> Self {
        self.response_data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VoidResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`response_code`](VoidResponseBuilder::response_code)
    /// - [`room_id`](VoidResponseBuilder::room_id)
    /// - [`is_success`](VoidResponseBuilder::is_success)
    /// - [`response_text`](VoidResponseBuilder::response_text)
    /// - [`response_data`](VoidResponseBuilder::response_data)
    pub fn build(self) -> Result<VoidResponse, BuildError> {
        Ok(VoidResponse {
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
