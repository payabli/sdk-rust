pub use crate::prelude::*;

/// Response for canceling a ghost card.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteGhostCardResponse {
    #[serde(rename = "isSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<IsSuccess>,
    #[serde(rename = "responseData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_data: Option<DeleteGhostCardResponseData>,
}

impl DeleteGhostCardResponse {
    pub fn builder() -> DeleteGhostCardResponseBuilder {
        <DeleteGhostCardResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteGhostCardResponseBuilder {
    is_success: Option<IsSuccess>,
    response_data: Option<DeleteGhostCardResponseData>,
}

impl DeleteGhostCardResponseBuilder {
    pub fn is_success(mut self, value: IsSuccess) -> Self {
        self.is_success = Some(value);
        self
    }

    pub fn response_data(mut self, value: DeleteGhostCardResponseData) -> Self {
        self.response_data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeleteGhostCardResponse`].
    pub fn build(self) -> Result<DeleteGhostCardResponse, BuildError> {
        Ok(DeleteGhostCardResponse {
            is_success: self.is_success,
            response_data: self.response_data,
        })
    }
}
