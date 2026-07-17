pub use crate::prelude::*;

/// Request for AddCustomer (body + query parameters)
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AddCustomerRequest {
    /// When `true`, the request creates a new customer record, regardless of whether customer identifiers match an existing customer.
    #[serde(rename = "forceCustomerCreation")]
    #[serde(skip)]
    pub force_customer_creation: Option<bool>,
    /// Flag indicating to replace existing customer with a new record. Possible values: 0 (don't replace), 1 (replace). Default is `0`.
    #[serde(rename = "replaceExisting")]
    #[serde(skip)]
    pub replace_existing: Option<i64>,
    #[serde(default)]
    pub body: CustomerData,
}

impl AddCustomerRequest {
    pub fn builder() -> AddCustomerRequestBuilder {
        <AddCustomerRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddCustomerRequestBuilder {
    force_customer_creation: Option<bool>,
    replace_existing: Option<i64>,
    body: Option<CustomerData>,
}

impl AddCustomerRequestBuilder {
    pub fn force_customer_creation(mut self, value: bool) -> Self {
        self.force_customer_creation = Some(value);
        self
    }

    pub fn replace_existing(mut self, value: i64) -> Self {
        self.replace_existing = Some(value);
        self
    }

    pub fn body(mut self, value: CustomerData) -> Self {
        self.body = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AddCustomerRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`body`](AddCustomerRequestBuilder::body)
    pub fn build(self) -> Result<AddCustomerRequest, BuildError> {
        Ok(AddCustomerRequest {
            force_customer_creation: self.force_customer_creation,
            replace_existing: self.replace_existing,
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
        })
    }
}
