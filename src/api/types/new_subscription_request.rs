pub use crate::prelude::*;

/// Request for NewSubscription (body + query parameters)
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct NewSubscriptionRequest {
    #[serde(rename = "forceCustomerCreation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_customer_creation: Option<ForceCustomerCreation>,
    #[serde(default)]
    pub body: SubscriptionRequestBody,
}

impl NewSubscriptionRequest {
    pub fn builder() -> NewSubscriptionRequestBuilder {
        <NewSubscriptionRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NewSubscriptionRequestBuilder {
    force_customer_creation: Option<ForceCustomerCreation>,
    body: Option<SubscriptionRequestBody>,
}

impl NewSubscriptionRequestBuilder {
    pub fn force_customer_creation(mut self, value: ForceCustomerCreation) -> Self {
        self.force_customer_creation = Some(value);
        self
    }

    pub fn body(mut self, value: SubscriptionRequestBody) -> Self {
        self.body = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`NewSubscriptionRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`body`](NewSubscriptionRequestBuilder::body)
    pub fn build(self) -> Result<NewSubscriptionRequest, BuildError> {
        Ok(NewSubscriptionRequest {
            force_customer_creation: self.force_customer_creation,
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
        })
    }
}
