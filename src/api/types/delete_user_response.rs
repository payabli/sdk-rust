pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteUserResponse {
    #[serde(rename = "responseText")]
    #[serde(default)]
    pub response_text: ResponseText,
}

impl DeleteUserResponse {
    pub fn builder() -> DeleteUserResponseBuilder {
        <DeleteUserResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteUserResponseBuilder {
    response_text: Option<ResponseText>,
}

impl DeleteUserResponseBuilder {
    pub fn response_text(mut self, value: ResponseText) -> Self {
        self.response_text = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeleteUserResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`response_text`](DeleteUserResponseBuilder::response_text)
    pub fn build(self) -> Result<DeleteUserResponse, BuildError> {
        Ok(DeleteUserResponse {
            response_text: self
                .response_text
                .ok_or_else(|| BuildError::missing_field("response_text"))?,
        })
    }
}
