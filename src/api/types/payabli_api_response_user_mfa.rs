pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PayabliApiResponseUserMfa {
    #[serde(rename = "inactiveTokenTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inactive_token_time: Option<i64>,
    #[serde(rename = "isSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<IsSuccess>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining: Option<i64>,
    #[serde(rename = "responseData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_data: Option<Responsedatanonobject>,
    #[serde(rename = "responseText")]
    #[serde(default)]
    pub response_text: ResponseText,
}

impl PayabliApiResponseUserMfa {
    pub fn builder() -> PayabliApiResponseUserMfaBuilder {
        <PayabliApiResponseUserMfaBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayabliApiResponseUserMfaBuilder {
    inactive_token_time: Option<i64>,
    is_success: Option<IsSuccess>,
    remaining: Option<i64>,
    response_data: Option<Responsedatanonobject>,
    response_text: Option<ResponseText>,
}

impl PayabliApiResponseUserMfaBuilder {
    pub fn inactive_token_time(mut self, value: i64) -> Self {
        self.inactive_token_time = Some(value);
        self
    }

    pub fn is_success(mut self, value: IsSuccess) -> Self {
        self.is_success = Some(value);
        self
    }

    pub fn remaining(mut self, value: i64) -> Self {
        self.remaining = Some(value);
        self
    }

    pub fn response_data(mut self, value: Responsedatanonobject) -> Self {
        self.response_data = Some(value);
        self
    }

    pub fn response_text(mut self, value: ResponseText) -> Self {
        self.response_text = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PayabliApiResponseUserMfa`].
    /// This method will fail if any of the following fields are not set:
    /// - [`response_text`](PayabliApiResponseUserMfaBuilder::response_text)
    pub fn build(self) -> Result<PayabliApiResponseUserMfa, BuildError> {
        Ok(PayabliApiResponseUserMfa {
            inactive_token_time: self.inactive_token_time,
            is_success: self.is_success,
            remaining: self.remaining,
            response_data: self.response_data,
            response_text: self
                .response_text
                .ok_or_else(|| BuildError::missing_field("response_text"))?,
        })
    }
}
