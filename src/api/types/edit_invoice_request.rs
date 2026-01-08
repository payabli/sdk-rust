pub use crate::prelude::*;

/// Request for EditInvoice (body + query parameters)
///
/// Request type for the EditInvoiceRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EditInvoiceRequest {
    /// When `true`, the request creates a new customer record, regardless of whether customer identifiers match an existing customer.
    #[serde(rename = "forceCustomerCreation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_customer_creation: Option<bool>,
    pub body: InvoiceDataRequest,
}
