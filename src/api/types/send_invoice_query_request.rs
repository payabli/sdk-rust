pub use crate::prelude::*;

/// Query parameters for SendInvoice
///
/// Request type for the SendInvoiceQueryRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SendInvoiceQueryRequest {
    /// When `true`, attaches a PDF version of invoice to the email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachfile: Option<bool>,
    /// Email address where the invoice will be sent to. If this parameter isn't included, Payabli uses the email address on file for the customer owner of the invoice.
    #[serde(rename = "mail2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail_2: Option<String>,
}
