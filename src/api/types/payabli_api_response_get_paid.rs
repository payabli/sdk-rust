pub use crate::prelude::*;

/// General response for GetPaid endpoint supporting multiple payment methods
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PayabliApiResponseGetPaid {
    #[serde(rename = "responseText")]
    #[serde(default)]
    pub response_text: ResponseText,
    #[serde(rename = "isSuccess")]
    #[serde(default)]
    pub is_success: IsSuccess,
    #[serde(rename = "pageIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_identifier: Option<String>,
    #[serde(rename = "responseData")]
    #[serde(default)]
    pub response_data: GetPaidResponseData,
}

impl PayabliApiResponseGetPaid {
    pub fn builder() -> PayabliApiResponseGetPaidBuilder {
        <PayabliApiResponseGetPaidBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayabliApiResponseGetPaidBuilder {
    response_text: Option<ResponseText>,
    is_success: Option<IsSuccess>,
    page_identifier: Option<String>,
    response_data: Option<GetPaidResponseData>,
}

impl PayabliApiResponseGetPaidBuilder {
    pub fn response_text(mut self, value: ResponseText) -> Self {
        self.response_text = Some(value);
        self
    }

    pub fn is_success(mut self, value: IsSuccess) -> Self {
        self.is_success = Some(value);
        self
    }

    pub fn page_identifier(mut self, value: impl Into<String>) -> Self {
        self.page_identifier = Some(value.into());
        self
    }

    pub fn response_data(mut self, value: GetPaidResponseData) -> Self {
        self.response_data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PayabliApiResponseGetPaid`].
    /// This method will fail if any of the following fields are not set:
    /// - [`response_text`](PayabliApiResponseGetPaidBuilder::response_text)
    /// - [`is_success`](PayabliApiResponseGetPaidBuilder::is_success)
    /// - [`response_data`](PayabliApiResponseGetPaidBuilder::response_data)
    pub fn build(self) -> Result<PayabliApiResponseGetPaid, BuildError> {
        Ok(PayabliApiResponseGetPaid {
            response_text: self
                .response_text
                .ok_or_else(|| BuildError::missing_field("response_text"))?,
            is_success: self
                .is_success
                .ok_or_else(|| BuildError::missing_field("is_success"))?,
            page_identifier: self.page_identifier,
            response_data: self
                .response_data
                .ok_or_else(|| BuildError::missing_field("response_data"))?,
        })
    }
}
