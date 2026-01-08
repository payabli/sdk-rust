pub use crate::prelude::*;

/// Request for AddInvoice (body + query parameters)
///
/// Request type for the AddInvoiceRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AddInvoiceRequest {
    #[serde(rename = "forceCustomerCreation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_customer_creation: Option<ForceCustomerCreation>,
    pub body: InvoiceDataRequest,
}
