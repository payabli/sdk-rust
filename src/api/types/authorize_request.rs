pub use crate::prelude::*;

/// Request for Authorize (body + query parameters)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuthorizeRequest {
    #[serde(rename = "forceCustomerCreation")]
    #[serde(skip_serializing)]
    pub force_customer_creation: Option<ForceCustomerCreation>,
    pub body: TransRequestBody,
}

impl AuthorizeRequest {
    pub fn builder() -> AuthorizeRequestBuilder {
        <AuthorizeRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AuthorizeRequestBuilder {
    force_customer_creation: Option<ForceCustomerCreation>,
    body: Option<TransRequestBody>,
}

impl AuthorizeRequestBuilder {
    pub fn force_customer_creation(mut self, value: ForceCustomerCreation) -> Self {
        self.force_customer_creation = Some(value);
        self
    }

    pub fn body(mut self, value: TransRequestBody) -> Self {
        self.body = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AuthorizeRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`body`](AuthorizeRequestBuilder::body)
    pub fn build(self) -> Result<AuthorizeRequest, BuildError> {
        Ok(AuthorizeRequest {
            force_customer_creation: self.force_customer_creation,
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
        })
    }
}
