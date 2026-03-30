pub use crate::prelude::*;

/// Response for creating a ghost card.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateGhostCardResponse {
    #[serde(rename = "isSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<IsSuccess>,
    #[serde(rename = "responseText")]
    #[serde(default)]
    pub response_text: ResponseText,
    #[serde(rename = "responseData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_data: Option<CreateGhostCardResponseData>,
}

impl CreateGhostCardResponse {
    pub fn builder() -> CreateGhostCardResponseBuilder {
        <CreateGhostCardResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateGhostCardResponseBuilder {
    is_success: Option<IsSuccess>,
    response_text: Option<ResponseText>,
    response_data: Option<CreateGhostCardResponseData>,
}

impl CreateGhostCardResponseBuilder {
    pub fn is_success(mut self, value: IsSuccess) -> Self {
        self.is_success = Some(value);
        self
    }

    pub fn response_text(mut self, value: ResponseText) -> Self {
        self.response_text = Some(value);
        self
    }

    pub fn response_data(mut self, value: CreateGhostCardResponseData) -> Self {
        self.response_data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateGhostCardResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`response_text`](CreateGhostCardResponseBuilder::response_text)
    pub fn build(self) -> Result<CreateGhostCardResponse, BuildError> {
        Ok(CreateGhostCardResponse {
            is_success: self.is_success,
            response_text: self
                .response_text
                .ok_or_else(|| BuildError::missing_field("response_text"))?,
            response_data: self.response_data,
        })
    }
}
