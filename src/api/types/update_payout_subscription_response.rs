pub use crate::prelude::*;

/// Success response
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdatePayoutSubscriptionResponse {
    #[serde(rename = "isSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<IsSuccess>,
    #[serde(rename = "responseText")]
    #[serde(default)]
    pub response_text: ResponseText,
    /// If `isSuccess` = true, this contains the payout subscription ID. When the subscription is paused, it also includes a description (for example, "42 paused").
    ///
    /// If `isSuccess` = false, this contains the reason for the failure.
    #[serde(rename = "responseData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_data: Option<String>,
    #[serde(rename = "customerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<CustomerId>,
}

impl UpdatePayoutSubscriptionResponse {
    pub fn builder() -> UpdatePayoutSubscriptionResponseBuilder {
        <UpdatePayoutSubscriptionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePayoutSubscriptionResponseBuilder {
    is_success: Option<IsSuccess>,
    response_text: Option<ResponseText>,
    response_data: Option<String>,
    customer_id: Option<CustomerId>,
}

impl UpdatePayoutSubscriptionResponseBuilder {
    pub fn is_success(mut self, value: IsSuccess) -> Self {
        self.is_success = Some(value);
        self
    }

    pub fn response_text(mut self, value: ResponseText) -> Self {
        self.response_text = Some(value);
        self
    }

    pub fn response_data(mut self, value: impl Into<String>) -> Self {
        self.response_data = Some(value.into());
        self
    }

    pub fn customer_id(mut self, value: CustomerId) -> Self {
        self.customer_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdatePayoutSubscriptionResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`response_text`](UpdatePayoutSubscriptionResponseBuilder::response_text)
    pub fn build(self) -> Result<UpdatePayoutSubscriptionResponse, BuildError> {
        Ok(UpdatePayoutSubscriptionResponse {
            is_success: self.is_success,
            response_text: self
                .response_text
                .ok_or_else(|| BuildError::missing_field("response_text"))?,
            response_data: self.response_data,
            customer_id: self.customer_id,
        })
    }
}
