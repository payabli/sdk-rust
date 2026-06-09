pub use crate::prelude::*;

/// Success response
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetPayoutSubscriptionResponse {
    #[serde(rename = "isSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<IsSuccess>,
    #[serde(rename = "responseText")]
    #[serde(default)]
    pub response_text: ResponseText,
    /// The payout subscription record.
    #[serde(rename = "responseData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_data: Option<PayoutSubscriptionQueryRecord>,
}

impl GetPayoutSubscriptionResponse {
    pub fn builder() -> GetPayoutSubscriptionResponseBuilder {
        <GetPayoutSubscriptionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetPayoutSubscriptionResponseBuilder {
    is_success: Option<IsSuccess>,
    response_text: Option<ResponseText>,
    response_data: Option<PayoutSubscriptionQueryRecord>,
}

impl GetPayoutSubscriptionResponseBuilder {
    pub fn is_success(mut self, value: IsSuccess) -> Self {
        self.is_success = Some(value);
        self
    }

    pub fn response_text(mut self, value: ResponseText) -> Self {
        self.response_text = Some(value);
        self
    }

    pub fn response_data(mut self, value: PayoutSubscriptionQueryRecord) -> Self {
        self.response_data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetPayoutSubscriptionResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`response_text`](GetPayoutSubscriptionResponseBuilder::response_text)
    pub fn build(self) -> Result<GetPayoutSubscriptionResponse, BuildError> {
        Ok(GetPayoutSubscriptionResponse {
            is_success: self.is_success,
            response_text: self
                .response_text
                .ok_or_else(|| BuildError::missing_field("response_text"))?,
            response_data: self.response_data,
        })
    }
}
