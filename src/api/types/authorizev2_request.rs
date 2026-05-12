pub use crate::prelude::*;

/// Request for authorizev2 (body + query parameters)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Authorizev2Request {
    #[serde(rename = "forceCustomerCreation")]
    #[serde(skip_serializing)]
    pub force_customer_creation: Option<ForceCustomerCreation>,
    pub body: TransRequestBody,
}

impl Authorizev2Request {
    pub fn builder() -> Authorizev2RequestBuilder {
        <Authorizev2RequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct Authorizev2RequestBuilder {
    force_customer_creation: Option<ForceCustomerCreation>,
    body: Option<TransRequestBody>,
}

impl Authorizev2RequestBuilder {
    pub fn force_customer_creation(mut self, value: ForceCustomerCreation) -> Self {
        self.force_customer_creation = Some(value);
        self
    }

    pub fn body(mut self, value: TransRequestBody) -> Self {
        self.body = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Authorizev2Request`].
    /// This method will fail if any of the following fields are not set:
    /// - [`body`](Authorizev2RequestBuilder::body)
    pub fn build(self) -> Result<Authorizev2Request, BuildError> {
        Ok(Authorizev2Request {
            force_customer_creation: self.force_customer_creation,
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
        })
    }
}
