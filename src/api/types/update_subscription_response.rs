pub use crate::prelude::*;

/// Success response
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateSubscriptionResponse {
    #[serde(rename = "isSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<IsSuccess>,
    /// If `isSuccess` = true, this contains the identifier of the subscription, and sometimes extra information, depending on what was updated.
    ///
    /// If `isSuccess` = false, this contains the reason for the failure.
    #[serde(rename = "responseData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_data: Option<String>,
    #[serde(rename = "responseText")]
    #[serde(default)]
    pub response_text: ResponseText,
    #[serde(rename = "customerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<CustomerId>,
}

impl UpdateSubscriptionResponse {
    pub fn builder() -> UpdateSubscriptionResponseBuilder {
        <UpdateSubscriptionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateSubscriptionResponseBuilder {
    is_success: Option<IsSuccess>,
    response_data: Option<String>,
    response_text: Option<ResponseText>,
    customer_id: Option<CustomerId>,
}

impl UpdateSubscriptionResponseBuilder {
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

    pub fn customer_id(mut self, value: CustomerId) -> Self {
        self.customer_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateSubscriptionResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`response_text`](UpdateSubscriptionResponseBuilder::response_text)
    pub fn build(self) -> Result<UpdateSubscriptionResponse, BuildError> {
        Ok(UpdateSubscriptionResponse {
            is_success: self.is_success,
            response_data: self.response_data,
            response_text: self
                .response_text
                .ok_or_else(|| BuildError::missing_field("response_text"))?,
            customer_id: self.customer_id,
        })
    }
}
