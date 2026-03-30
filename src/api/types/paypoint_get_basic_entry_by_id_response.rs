pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetBasicEntryByIdResponse {
    #[serde(rename = "isSuccess")]
    #[serde(default)]
    pub is_success: IsSuccess,
    #[serde(rename = "pageIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_identifier: Option<PageIdentifier>,
    #[serde(rename = "responseCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_code: Option<Responsecode>,
    #[serde(rename = "responseData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_data: Option<PaypointEntryConfig>,
    #[serde(rename = "responseText")]
    #[serde(default)]
    pub response_text: ResponseText,
}

impl GetBasicEntryByIdResponse {
    pub fn builder() -> GetBasicEntryByIdResponseBuilder {
        <GetBasicEntryByIdResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetBasicEntryByIdResponseBuilder {
    is_success: Option<IsSuccess>,
    page_identifier: Option<PageIdentifier>,
    response_code: Option<Responsecode>,
    response_data: Option<PaypointEntryConfig>,
    response_text: Option<ResponseText>,
}

impl GetBasicEntryByIdResponseBuilder {
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

    pub fn response_data(mut self, value: PaypointEntryConfig) -> Self {
        self.response_data = Some(value);
        self
    }

    pub fn response_text(mut self, value: ResponseText) -> Self {
        self.response_text = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetBasicEntryByIdResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`is_success`](GetBasicEntryByIdResponseBuilder::is_success)
    /// - [`response_text`](GetBasicEntryByIdResponseBuilder::response_text)
    pub fn build(self) -> Result<GetBasicEntryByIdResponse, BuildError> {
        Ok(GetBasicEntryByIdResponse {
            is_success: self
                .is_success
                .ok_or_else(|| BuildError::missing_field("is_success"))?,
            page_identifier: self.page_identifier,
            response_code: self.response_code,
            response_data: self.response_data,
            response_text: self
                .response_text
                .ok_or_else(|| BuildError::missing_field("response_text"))?,
        })
    }
}
