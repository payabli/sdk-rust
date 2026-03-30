pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct InvoiceDataRequest {
    /// Object describing the customer/payor. Required for POST requests. Which fields are required depends on the paypoint's custom identifier settings.
    #[serde(rename = "customerData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_data: Option<PayorDataRequest>,
    /// Object describing the invoice. Required for POST requests.
    #[serde(rename = "invoiceData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_data: Option<BillData>,
    /// Object with options for scheduled invoices.
    #[serde(rename = "scheduledOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_options: Option<BillOptions>,
}

impl InvoiceDataRequest {
    pub fn builder() -> InvoiceDataRequestBuilder {
        <InvoiceDataRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoiceDataRequestBuilder {
    customer_data: Option<PayorDataRequest>,
    invoice_data: Option<BillData>,
    scheduled_options: Option<BillOptions>,
}

impl InvoiceDataRequestBuilder {
    pub fn customer_data(mut self, value: PayorDataRequest) -> Self {
        self.customer_data = Some(value);
        self
    }

    pub fn invoice_data(mut self, value: BillData) -> Self {
        self.invoice_data = Some(value);
        self
    }

    pub fn scheduled_options(mut self, value: BillOptions) -> Self {
        self.scheduled_options = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvoiceDataRequest`].
    pub fn build(self) -> Result<InvoiceDataRequest, BuildError> {
        Ok(InvoiceDataRequest {
            customer_data: self.customer_data,
            invoice_data: self.invoice_data,
            scheduled_options: self.scheduled_options,
        })
    }
}
