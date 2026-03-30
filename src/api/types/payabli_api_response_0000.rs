pub use crate::prelude::*;

/// The response for canceling a single payout transaction.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PayabliApiResponse0000 {
    #[serde(rename = "isSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<IsSuccess>,
    #[serde(rename = "responseText")]
    #[serde(default)]
    pub response_text: ResponseText,
    #[serde(rename = "pageIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_identifier: Option<PageIdentifier>,
    #[serde(rename = "responseCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_code: Option<Responsecode>,
    #[serde(rename = "responseData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_data: Option<PayabliApiResponse0ResponseData>,
}

impl PayabliApiResponse0000 {
    pub fn builder() -> PayabliApiResponse0000Builder {
        <PayabliApiResponse0000Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayabliApiResponse0000Builder {
    is_success: Option<IsSuccess>,
    response_text: Option<ResponseText>,
    page_identifier: Option<PageIdentifier>,
    response_code: Option<Responsecode>,
    response_data: Option<PayabliApiResponse0ResponseData>,
}

impl PayabliApiResponse0000Builder {
    pub fn is_success(mut self, value: IsSuccess) -> Self {
        self.is_success = Some(value);
        self
    }

    pub fn response_text(mut self, value: ResponseText) -> Self {
        self.response_text = Some(value);
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

    pub fn response_data(mut self, value: PayabliApiResponse0ResponseData) -> Self {
        self.response_data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PayabliApiResponse0000`].
    /// This method will fail if any of the following fields are not set:
    /// - [`response_text`](PayabliApiResponse0000Builder::response_text)
    pub fn build(self) -> Result<PayabliApiResponse0000, BuildError> {
        Ok(PayabliApiResponse0000 {
            is_success: self.is_success,
            response_text: self
                .response_text
                .ok_or_else(|| BuildError::missing_field("response_text"))?,
            page_identifier: self.page_identifier,
            response_code: self.response_code,
            response_data: self.response_data,
        })
    }
}
