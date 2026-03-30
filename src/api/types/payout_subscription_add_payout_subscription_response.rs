pub use crate::prelude::*;

/// Success response
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AddPayoutSubscriptionResponse {
    #[serde(rename = "isSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<IsSuccess>,
    #[serde(rename = "responseText")]
    #[serde(default)]
    pub response_text: ResponseText,
    /// The identifier of the newly created payout subscription.
    #[serde(rename = "responseData")]
    #[serde(default)]
    pub response_data: i64,
    /// The identifier of the vendor associated with the payout subscription.
    #[serde(rename = "customerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<CustomerId>,
}

impl AddPayoutSubscriptionResponse {
    pub fn builder() -> AddPayoutSubscriptionResponseBuilder {
        <AddPayoutSubscriptionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddPayoutSubscriptionResponseBuilder {
    is_success: Option<IsSuccess>,
    response_text: Option<ResponseText>,
    response_data: Option<i64>,
    customer_id: Option<CustomerId>,
}

impl AddPayoutSubscriptionResponseBuilder {
    pub fn is_success(mut self, value: IsSuccess) -> Self {
        self.is_success = Some(value);
        self
    }

    pub fn response_text(mut self, value: ResponseText) -> Self {
        self.response_text = Some(value);
        self
    }

    pub fn response_data(mut self, value: i64) -> Self {
        self.response_data = Some(value);
        self
    }

    pub fn customer_id(mut self, value: CustomerId) -> Self {
        self.customer_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AddPayoutSubscriptionResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`response_text`](AddPayoutSubscriptionResponseBuilder::response_text)
    /// - [`response_data`](AddPayoutSubscriptionResponseBuilder::response_data)
    pub fn build(self) -> Result<AddPayoutSubscriptionResponse, BuildError> {
        Ok(AddPayoutSubscriptionResponse {
            is_success: self.is_success,
            response_text: self
                .response_text
                .ok_or_else(|| BuildError::missing_field("response_text"))?,
            response_data: self
                .response_data
                .ok_or_else(|| BuildError::missing_field("response_data"))?,
            customer_id: self.customer_id,
        })
    }
}
