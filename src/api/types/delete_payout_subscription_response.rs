pub use crate::prelude::*;

/// Success response
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeletePayoutSubscriptionResponse {
    #[serde(rename = "isSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<IsSuccess>,
    /// If `isSuccess` = true, this contains the identifier of the payout subscription.
    ///
    /// If `isSuccess` = false, this contains the reason for the failure.
    #[serde(rename = "responseData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_data: Option<String>,
    #[serde(rename = "responseText")]
    #[serde(default)]
    pub response_text: ResponseText,
}

impl DeletePayoutSubscriptionResponse {
    pub fn builder() -> DeletePayoutSubscriptionResponseBuilder {
        <DeletePayoutSubscriptionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeletePayoutSubscriptionResponseBuilder {
    is_success: Option<IsSuccess>,
    response_data: Option<String>,
    response_text: Option<ResponseText>,
}

impl DeletePayoutSubscriptionResponseBuilder {
    pub fn is_success(mut self, value: IsSuccess) -> Self {
        self.is_success = Some(value);
        self
    }

    pub fn response_data(mut self, value: impl Into<String>) -> Self {
        self.response_data = Some(value.into());
        self
    }

    pub fn response_text(mut self, value: ResponseText) -> Self {
        self.response_text = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeletePayoutSubscriptionResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`response_text`](DeletePayoutSubscriptionResponseBuilder::response_text)
    pub fn build(self) -> Result<DeletePayoutSubscriptionResponse, BuildError> {
        Ok(DeletePayoutSubscriptionResponse {
            is_success: self.is_success,
            response_data: self.response_data,
            response_text: self
                .response_text
                .ok_or_else(|| BuildError::missing_field("response_text"))?,
        })
    }
}
