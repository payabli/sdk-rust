pub use crate::prelude::*;

/// General response for certain `moneyIn` and `moneyOut` endpoints.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PayabliApiResponse0 {
    #[serde(rename = "isSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<IsSuccess>,
    #[serde(rename = "pageIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_identifier: Option<PageIdentifier>,
    #[serde(rename = "responseData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_data: Option<PayabliApiResponse0ResponseData>,
    #[serde(rename = "responseText")]
    #[serde(default)]
    pub response_text: ResponseText,
}

impl PayabliApiResponse0 {
    pub fn builder() -> PayabliApiResponse0Builder {
        <PayabliApiResponse0Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayabliApiResponse0Builder {
    is_success: Option<IsSuccess>,
    page_identifier: Option<PageIdentifier>,
    response_data: Option<PayabliApiResponse0ResponseData>,
    response_text: Option<ResponseText>,
}

impl PayabliApiResponse0Builder {
    pub fn is_success(mut self, value: IsSuccess) -> Self {
        self.is_success = Some(value);
        self
    }

    pub fn page_identifier(mut self, value: PageIdentifier) -> Self {
        self.page_identifier = Some(value);
        self
    }

    pub fn response_data(mut self, value: PayabliApiResponse0ResponseData) -> Self {
        self.response_data = Some(value);
        self
    }

    pub fn response_text(mut self, value: ResponseText) -> Self {
        self.response_text = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PayabliApiResponse0`].
    /// This method will fail if any of the following fields are not set:
    /// - [`response_text`](PayabliApiResponse0Builder::response_text)
    pub fn build(self) -> Result<PayabliApiResponse0, BuildError> {
        Ok(PayabliApiResponse0 {
            is_success: self.is_success,
            page_identifier: self.page_identifier,
            response_data: self.response_data,
            response_text: self
                .response_text
                .ok_or_else(|| BuildError::missing_field("response_text"))?,
        })
    }
}
