pub use crate::prelude::*;

/// Shape returned by every Payabli API error response. The `responseData`
/// object carries human-readable error context.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PayabliErrorBody {
    /// Always `false` for error responses.
    #[serde(rename = "isSuccess")]
    #[serde(default)]
    pub is_success: bool,
    /// Code for the response. Learn more in
    /// [API Response Codes](/developers/api-reference/api-responses).
    #[serde(rename = "responseCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_code: Option<i64>,
    /// Error text describing what went wrong.
    #[serde(rename = "responseText")]
    #[serde(default)]
    pub response_text: String,
    /// Object with detailed error context.
    #[serde(rename = "responseData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_data: Option<PayabliErrorBodyResponseData>,
}

impl PayabliErrorBody {
    pub fn builder() -> PayabliErrorBodyBuilder {
        <PayabliErrorBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayabliErrorBodyBuilder {
    is_success: Option<bool>,
    response_code: Option<i64>,
    response_text: Option<String>,
    response_data: Option<PayabliErrorBodyResponseData>,
}

impl PayabliErrorBodyBuilder {
    pub fn is_success(mut self, value: bool) -> Self {
        self.is_success = Some(value);
        self
    }

    pub fn response_code(mut self, value: i64) -> Self {
        self.response_code = Some(value);
        self
    }

    pub fn response_text(mut self, value: impl Into<String>) -> Self {
        self.response_text = Some(value.into());
        self
    }

    pub fn response_data(mut self, value: PayabliErrorBodyResponseData) -> Self {
        self.response_data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PayabliErrorBody`].
    /// This method will fail if any of the following fields are not set:
    /// - [`is_success`](PayabliErrorBodyBuilder::is_success)
    /// - [`response_text`](PayabliErrorBodyBuilder::response_text)
    pub fn build(self) -> Result<PayabliErrorBody, BuildError> {
        Ok(PayabliErrorBody {
            is_success: self
                .is_success
                .ok_or_else(|| BuildError::missing_field("is_success"))?,
            response_code: self.response_code,
            response_text: self
                .response_text
                .ok_or_else(|| BuildError::missing_field("response_text"))?,
            response_data: self.response_data,
        })
    }
}
