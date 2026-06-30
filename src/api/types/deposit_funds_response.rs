pub use crate::prelude::*;

/// Response for a deposit funds request.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DepositFundsResponse {
    #[serde(rename = "isSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<IsSuccess>,
    #[serde(rename = "responseText")]
    #[serde(default)]
    pub response_text: ResponseText,
    /// The object containing the response data.
    #[serde(rename = "responseData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_data: Option<Responsedata>,
    /// Auxiliary validation used internally by payment pages and components.
    #[serde(rename = "pageIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_identifier: Option<PageIdentifier>,
}

impl DepositFundsResponse {
    pub fn builder() -> DepositFundsResponseBuilder {
        <DepositFundsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DepositFundsResponseBuilder {
    is_success: Option<IsSuccess>,
    response_text: Option<ResponseText>,
    response_data: Option<Responsedata>,
    page_identifier: Option<PageIdentifier>,
}

impl DepositFundsResponseBuilder {
    pub fn is_success(mut self, value: IsSuccess) -> Self {
        self.is_success = Some(value);
        self
    }

    pub fn response_text(mut self, value: ResponseText) -> Self {
        self.response_text = Some(value);
        self
    }

    pub fn response_data(mut self, value: Responsedata) -> Self {
        self.response_data = Some(value);
        self
    }

    pub fn page_identifier(mut self, value: PageIdentifier) -> Self {
        self.page_identifier = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DepositFundsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`response_text`](DepositFundsResponseBuilder::response_text)
    pub fn build(self) -> Result<DepositFundsResponse, BuildError> {
        Ok(DepositFundsResponse {
            is_success: self.is_success,
            response_text: self
                .response_text
                .ok_or_else(|| BuildError::missing_field("response_text"))?,
            response_data: self.response_data,
            page_identifier: self.page_identifier,
        })
    }
}
