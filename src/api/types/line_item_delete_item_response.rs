pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteItemResponse {
    #[serde(rename = "isSuccess")]
    #[serde(default)]
    pub is_success: IsSuccess,
    #[serde(rename = "responseText")]
    #[serde(default)]
    pub response_text: ResponseText,
}

impl DeleteItemResponse {
    pub fn builder() -> DeleteItemResponseBuilder {
        <DeleteItemResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteItemResponseBuilder {
    is_success: Option<IsSuccess>,
    response_text: Option<ResponseText>,
}

impl DeleteItemResponseBuilder {
    pub fn is_success(mut self, value: IsSuccess) -> Self {
        self.is_success = Some(value);
        self
    }

    pub fn response_text(mut self, value: ResponseText) -> Self {
        self.response_text = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeleteItemResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`is_success`](DeleteItemResponseBuilder::is_success)
    /// - [`response_text`](DeleteItemResponseBuilder::response_text)
    pub fn build(self) -> Result<DeleteItemResponse, BuildError> {
        Ok(DeleteItemResponse {
            is_success: self
                .is_success
                .ok_or_else(|| BuildError::missing_field("is_success"))?,
            response_text: self
                .response_text
                .ok_or_else(|| BuildError::missing_field("response_text"))?,
        })
    }
}
