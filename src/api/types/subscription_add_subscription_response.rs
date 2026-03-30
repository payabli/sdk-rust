pub use crate::prelude::*;

/// Success response
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AddSubscriptionResponse {
    #[serde(rename = "customerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<CustomerId>,
    #[serde(rename = "responseText")]
    #[serde(default)]
    pub response_text: ResponseText,
    #[serde(rename = "isSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<IsSuccess>,
    /// The identifier of the newly created subscription.
    #[serde(rename = "responseData")]
    #[serde(default)]
    pub response_data: i64,
}

impl AddSubscriptionResponse {
    pub fn builder() -> AddSubscriptionResponseBuilder {
        <AddSubscriptionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddSubscriptionResponseBuilder {
    customer_id: Option<CustomerId>,
    response_text: Option<ResponseText>,
    is_success: Option<IsSuccess>,
    response_data: Option<i64>,
}

impl AddSubscriptionResponseBuilder {
    pub fn customer_id(mut self, value: CustomerId) -> Self {
        self.customer_id = Some(value);
        self
    }

    pub fn response_text(mut self, value: ResponseText) -> Self {
        self.response_text = Some(value);
        self
    }

    pub fn is_success(mut self, value: IsSuccess) -> Self {
        self.is_success = Some(value);
        self
    }

    pub fn response_data(mut self, value: i64) -> Self {
        self.response_data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AddSubscriptionResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`response_text`](AddSubscriptionResponseBuilder::response_text)
    /// - [`response_data`](AddSubscriptionResponseBuilder::response_data)
    pub fn build(self) -> Result<AddSubscriptionResponse, BuildError> {
        Ok(AddSubscriptionResponse {
            customer_id: self.customer_id,
            response_text: self
                .response_text
                .ok_or_else(|| BuildError::missing_field("response_text"))?,
            is_success: self.is_success,
            response_data: self
                .response_data
                .ok_or_else(|| BuildError::missing_field("response_data"))?,
        })
    }
}
