pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MigratePaypointResponse {
    #[serde(rename = "isSuccess")]
    #[serde(default)]
    pub is_success: IsSuccess,
    #[serde(rename = "responseCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_code: Option<Responsecode>,
    #[serde(rename = "responseText")]
    #[serde(default)]
    pub response_text: ResponseText,
}

impl MigratePaypointResponse {
    pub fn builder() -> MigratePaypointResponseBuilder {
        <MigratePaypointResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MigratePaypointResponseBuilder {
    is_success: Option<IsSuccess>,
    response_code: Option<Responsecode>,
    response_text: Option<ResponseText>,
}

impl MigratePaypointResponseBuilder {
    pub fn is_success(mut self, value: IsSuccess) -> Self {
        self.is_success = Some(value);
        self
    }

    pub fn response_code(mut self, value: Responsecode) -> Self {
        self.response_code = Some(value);
        self
    }

    pub fn response_text(mut self, value: ResponseText) -> Self {
        self.response_text = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MigratePaypointResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`is_success`](MigratePaypointResponseBuilder::is_success)
    /// - [`response_text`](MigratePaypointResponseBuilder::response_text)
    pub fn build(self) -> Result<MigratePaypointResponse, BuildError> {
        Ok(MigratePaypointResponse {
            is_success: self
                .is_success
                .ok_or_else(|| BuildError::missing_field("is_success"))?,
            response_code: self.response_code,
            response_text: self
                .response_text
                .ok_or_else(|| BuildError::missing_field("response_text"))?,
        })
    }
}
