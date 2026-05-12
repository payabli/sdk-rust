pub use crate::prelude::*;

/// Request for getpaidv2 (body + query parameters)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Getpaidv2Request {
    #[serde(rename = "achValidation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_validation: Option<AchValidation>,
    #[serde(rename = "forceCustomerCreation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_customer_creation: Option<ForceCustomerCreation>,
    pub body: TransRequestBody,
}

impl Getpaidv2Request {
    pub fn builder() -> Getpaidv2RequestBuilder {
        <Getpaidv2RequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct Getpaidv2RequestBuilder {
    ach_validation: Option<AchValidation>,
    force_customer_creation: Option<ForceCustomerCreation>,
    body: Option<TransRequestBody>,
}

impl Getpaidv2RequestBuilder {
    pub fn ach_validation(mut self, value: AchValidation) -> Self {
        self.ach_validation = Some(value);
        self
    }

    pub fn force_customer_creation(mut self, value: ForceCustomerCreation) -> Self {
        self.force_customer_creation = Some(value);
        self
    }

    pub fn body(mut self, value: TransRequestBody) -> Self {
        self.body = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Getpaidv2Request`].
    /// This method will fail if any of the following fields are not set:
    /// - [`body`](Getpaidv2RequestBuilder::body)
    pub fn build(self) -> Result<Getpaidv2Request, BuildError> {
        Ok(Getpaidv2Request {
            ach_validation: self.ach_validation,
            force_customer_creation: self.force_customer_creation,
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
        })
    }
}
