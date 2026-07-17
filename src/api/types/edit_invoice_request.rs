pub use crate::prelude::*;

/// Request for EditInvoice (body + query parameters)
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct EditInvoiceRequest {
    /// When `true`, the request creates a new customer record, regardless of whether customer identifiers match an existing customer.
    #[serde(rename = "forceCustomerCreation")]
    #[serde(skip)]
    pub force_customer_creation: Option<bool>,
    #[serde(default)]
    pub body: InvoiceDataRequest,
}

impl EditInvoiceRequest {
    pub fn builder() -> EditInvoiceRequestBuilder {
        <EditInvoiceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EditInvoiceRequestBuilder {
    force_customer_creation: Option<bool>,
    body: Option<InvoiceDataRequest>,
}

impl EditInvoiceRequestBuilder {
    pub fn force_customer_creation(mut self, value: bool) -> Self {
        self.force_customer_creation = Some(value);
        self
    }

    pub fn body(mut self, value: InvoiceDataRequest) -> Self {
        self.body = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EditInvoiceRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`body`](EditInvoiceRequestBuilder::body)
    pub fn build(self) -> Result<EditInvoiceRequest, BuildError> {
        Ok(EditInvoiceRequest {
            force_customer_creation: self.force_customer_creation,
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
        })
    }
}
