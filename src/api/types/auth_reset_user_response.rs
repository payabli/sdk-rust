pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AuthResetUserResponse {
    #[serde(rename = "isSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<IsSuccess>,
    #[serde(rename = "responseText")]
    #[serde(default)]
    pub response_text: ResponseText,
}

impl AuthResetUserResponse {
    pub fn builder() -> AuthResetUserResponseBuilder {
        <AuthResetUserResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AuthResetUserResponseBuilder {
    is_success: Option<IsSuccess>,
    response_text: Option<ResponseText>,
}

impl AuthResetUserResponseBuilder {
    pub fn is_success(mut self, value: IsSuccess) -> Self {
        self.is_success = Some(value);
        self
    }

    pub fn response_text(mut self, value: ResponseText) -> Self {
        self.response_text = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AuthResetUserResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`response_text`](AuthResetUserResponseBuilder::response_text)
    pub fn build(self) -> Result<AuthResetUserResponse, BuildError> {
        Ok(AuthResetUserResponse {
            is_success: self.is_success,
            response_text: self
                .response_text
                .ok_or_else(|| BuildError::missing_field("response_text"))?,
        })
    }
}
