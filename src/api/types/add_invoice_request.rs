pub use crate::prelude::*;

/// Request for AddInvoice (body + query parameters)
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AddInvoiceRequest {
    /// When `true`, the request creates a new customer record, regardless of whether customer identifiers match an existing customer. Defaults to `false`.
    #[serde(rename = "forceCustomerCreation")]
    #[serde(skip)]
    pub force_customer_creation: Option<ForceCustomerCreation>,
    #[serde(default)]
    pub body: InvoiceDataRequest,
}

impl AddInvoiceRequest {
    pub fn builder() -> AddInvoiceRequestBuilder {
        <AddInvoiceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddInvoiceRequestBuilder {
    force_customer_creation: Option<ForceCustomerCreation>,
    body: Option<InvoiceDataRequest>,
}

impl AddInvoiceRequestBuilder {
    pub fn force_customer_creation(mut self, value: ForceCustomerCreation) -> Self {
        self.force_customer_creation = Some(value);
        self
    }

    pub fn body(mut self, value: InvoiceDataRequest) -> Self {
        self.body = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AddInvoiceRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`body`](AddInvoiceRequestBuilder::body)
    pub fn build(self) -> Result<AddInvoiceRequest, BuildError> {
        Ok(AddInvoiceRequest {
            force_customer_creation: self.force_customer_creation,
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
        })
    }
}
