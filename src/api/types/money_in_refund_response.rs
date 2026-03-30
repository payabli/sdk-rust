pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RefundResponse {
    #[serde(rename = "responseText")]
    #[serde(default)]
    pub response_text: ResponseText,
    #[serde(rename = "isSuccess")]
    #[serde(default)]
    pub is_success: IsSuccess,
    #[serde(rename = "responseData")]
    #[serde(default)]
    pub response_data: ResponseDataRefunds,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pageidentifier: Option<PageIdentifier>,
}

impl RefundResponse {
    pub fn builder() -> RefundResponseBuilder {
        <RefundResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RefundResponseBuilder {
    response_text: Option<ResponseText>,
    is_success: Option<IsSuccess>,
    response_data: Option<ResponseDataRefunds>,
    pageidentifier: Option<PageIdentifier>,
}

impl RefundResponseBuilder {
    pub fn response_text(mut self, value: ResponseText) -> Self {
        self.response_text = Some(value);
        self
    }

    pub fn is_success(mut self, value: IsSuccess) -> Self {
        self.is_success = Some(value);
        self
    }

    pub fn response_data(mut self, value: ResponseDataRefunds) -> Self {
        self.response_data = Some(value);
        self
    }

    pub fn pageidentifier(mut self, value: PageIdentifier) -> Self {
        self.pageidentifier = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RefundResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`response_text`](RefundResponseBuilder::response_text)
    /// - [`is_success`](RefundResponseBuilder::is_success)
    /// - [`response_data`](RefundResponseBuilder::response_data)
    pub fn build(self) -> Result<RefundResponse, BuildError> {
        Ok(RefundResponse {
            response_text: self
                .response_text
                .ok_or_else(|| BuildError::missing_field("response_text"))?,
            is_success: self
                .is_success
                .ok_or_else(|| BuildError::missing_field("is_success"))?,
            response_data: self
                .response_data
                .ok_or_else(|| BuildError::missing_field("response_data"))?,
            pageidentifier: self.pageidentifier,
        })
    }
}
