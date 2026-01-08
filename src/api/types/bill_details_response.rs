pub use crate::prelude::*;

/// Response object for bill details. Contains basic information about a bill.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BillDetailsResponse {
    #[serde(rename = "billId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_id: Option<BillId>,
    /// Lot number of the bill.
    #[serde(rename = "lotNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lot_number: Option<String>,
    /// Custom number identifying the bill.
    #[serde(rename = "invoiceNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_number: Option<InvoiceNumber>,
    /// Net Amount owed in bill. Required when adding a bill.
    #[serde(rename = "netAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_amount: Option<NetAmountstring>,
    /// Bill discount amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<String>,
    /// Bill due date in format YYYY-MM-DD or MM/DD/YYYY.
    #[serde(rename = "dueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<Datenullable>,
    /// Bill date in format YYYY-MM-DD or MM/DD/YYYY.
    #[serde(rename = "invoiceDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_date: Option<Datenullable>,
    /// Any comments about bill. **For managed payouts, this field has a limit of 100 characters**.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<Comments>,
}
