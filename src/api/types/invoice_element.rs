pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InvoiceElement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<Enabled>,
    /// Link to invoice
    #[serde(rename = "invoiceLink")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_link: Option<LabelElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
    /// Link to view invoice details
    #[serde(rename = "viewInvoiceDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_invoice_details: Option<LabelElement>,
}
