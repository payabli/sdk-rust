pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PayabliApiResponsePaylinks {
    #[serde(rename = "isSuccess")]
    #[serde(default)]
    pub is_success: IsSuccess,
    #[serde(rename = "pageIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_identifier: Option<PageIdentifier>,
    #[serde(rename = "responseCode")]
    #[serde(default)]
    pub response_code: Responsecode,
    /// The paylink ID or error details.
    #[serde(rename = "responseData")]
    #[serde(default)]
    pub response_data: Responsedata,
    #[serde(rename = "responseText")]
    #[serde(default)]
    pub response_text: ResponseText,
}

impl PayabliApiResponsePaylinks {
    pub fn builder() -> PayabliApiResponsePaylinksBuilder {
        <PayabliApiResponsePaylinksBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayabliApiResponsePaylinksBuilder {
    is_success: Option<IsSuccess>,
    page_identifier: Option<PageIdentifier>,
    response_code: Option<Responsecode>,
    response_data: Option<Responsedata>,
    response_text: Option<ResponseText>,
}

impl PayabliApiResponsePaylinksBuilder {
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

    pub fn response_data(mut self, value: Responsedata) -> Self {
        self.response_data = Some(value);
        self
    }

    pub fn response_text(mut self, value: ResponseText) -> Self {
        self.response_text = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PayabliApiResponsePaylinks`].
    /// This method will fail if any of the following fields are not set:
    /// - [`is_success`](PayabliApiResponsePaylinksBuilder::is_success)
    /// - [`response_code`](PayabliApiResponsePaylinksBuilder::response_code)
    /// - [`response_data`](PayabliApiResponsePaylinksBuilder::response_data)
    /// - [`response_text`](PayabliApiResponsePaylinksBuilder::response_text)
    pub fn build(self) -> Result<PayabliApiResponsePaylinks, BuildError> {
        Ok(PayabliApiResponsePaylinks {
            is_success: self
                .is_success
                .ok_or_else(|| BuildError::missing_field("is_success"))?,
            page_identifier: self.page_identifier,
            response_code: self
                .response_code
                .ok_or_else(|| BuildError::missing_field("response_code"))?,
            response_data: self
                .response_data
                .ok_or_else(|| BuildError::missing_field("response_data"))?,
            response_text: self
                .response_text
                .ok_or_else(|| BuildError::missing_field("response_text"))?,
        })
    }
}
