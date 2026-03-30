pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PayabliApiResponseError400 {
    /// Boolean indicating whether the operation was successful. A `true` value indicates success. A `false` value indicates failure.
    #[serde(rename = "isSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pageidentifier: Option<PageIdentifier>,
    /// A code that indicates the operation's failure reason. See [API Response Codes](https://docs.payabli.com/api-reference/api-responses) for a full reference.
    #[serde(rename = "responseCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_code: Option<i64>,
    /// Describes the reason for a failed operation and how to resolve it.
    #[serde(rename = "responseData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_data: Option<PayabliApiResponseError400ResponseData>,
    /// Response text for operation: 'Success' or 'Declined'.
    #[serde(rename = "responseText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_text: Option<String>,
}

impl PayabliApiResponseError400 {
    pub fn builder() -> PayabliApiResponseError400Builder {
        <PayabliApiResponseError400Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayabliApiResponseError400Builder {
    is_success: Option<bool>,
    pageidentifier: Option<PageIdentifier>,
    response_code: Option<i64>,
    response_data: Option<PayabliApiResponseError400ResponseData>,
    response_text: Option<String>,
}

impl PayabliApiResponseError400Builder {
    pub fn is_success(mut self, value: bool) -> Self {
        self.is_success = Some(value);
        self
    }

    pub fn pageidentifier(mut self, value: PageIdentifier) -> Self {
        self.pageidentifier = Some(value);
        self
    }

    pub fn response_code(mut self, value: i64) -> Self {
        self.response_code = Some(value);
        self
    }

    pub fn response_data(mut self, value: PayabliApiResponseError400ResponseData) -> Self {
        self.response_data = Some(value);
        self
    }

    pub fn response_text(mut self, value: impl Into<String>) -> Self {
        self.response_text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PayabliApiResponseError400`].
    pub fn build(self) -> Result<PayabliApiResponseError400, BuildError> {
        Ok(PayabliApiResponseError400 {
            is_success: self.is_success,
            pageidentifier: self.pageidentifier,
            response_code: self.response_code,
            response_data: self.response_data,
            response_text: self.response_text,
        })
    }
}
